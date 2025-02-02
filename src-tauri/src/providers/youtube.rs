// Moosync
// Copyright (C) 2024, 2025  Moosync <support@moosync.app>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use async_trait::async_trait;
use futures::channel::mpsc::UnboundedSender;
use futures::SinkExt;
use google_youtube3::api::{Channel, ChannelSnippet, Playlist, PlaylistSnippet, Video};
use google_youtube3::hyper_rustls::HttpsConnector;
use google_youtube3::hyper_util::client::legacy::connect::HttpConnector;
use google_youtube3::{hyper_rustls, hyper_util, YouTube};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeVerifier, RedirectUrl, TokenUrl};
use preferences::preferences::PreferenceConfig;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use types::entities::{
    EntityInfo, QueryableAlbum, QueryableArtist, QueryablePlaylist, SearchResult,
};
use types::errors::{MoosyncError, Result};
use types::providers::generic::{Pagination, ProviderStatus};
use types::songs::{QueryableSong, Song, SongType};
use types::ui::extensions::{ContextMenuReturnType, ExtensionProviderScope};
use types::{oauth::OAuth2Client, providers::generic::GenericProvider};
use url::Url;
use youtube::youtube::YoutubeScraper;

use crate::oauth::handler::OAuthHandler;

use super::common::{authorize, login, refresh_login, LoginArgs, TokenHolder};

macro_rules! search_and_parse {
    ($client:expr, $term:expr, $type:expr, $process_fn:expr) => {{
        let (_, search_results) = $client
            .search()
            .list(&vec!["snippet".into()])
            .add_type($type)
            .q($term)
            .max_results(50)
            .doit()
            .await?;

        search_results.items.map_or(vec![], |items| {
            items.into_iter().filter_map($process_fn).collect()
        })
    }};
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArtistExtraInfo {
    artist_id: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct YoutubeExtraInfo {
    youtube: ArtistExtraInfo,
}

#[derive(Debug, Clone, Default)]
struct YoutubeConfig {
    client_secret: Option<String>,
    client_id: Option<String>,
    redirect_uri: &'static str,
    scopes: Vec<&'static str>,
    tokens: Option<TokenHolder>,
}

pub struct YoutubeProvider {
    app: AppHandle,
    config: YoutubeConfig,
    verifier: Option<(OAuth2Client, PkceCodeVerifier, CsrfToken)>,
    api_client: Option<YouTube<HttpsConnector<HttpConnector>>>,
    status_tx: UnboundedSender<ProviderStatus>,
}

impl std::fmt::Debug for YoutubeProvider {
    #[tracing::instrument(level = "trace", skip(self, f))]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <YoutubeConfig as std::fmt::Debug>::fmt(&self.config, f)
    }
}

impl YoutubeProvider {
    #[tracing::instrument(level = "trace", skip(app, status_tx))]
    pub fn new(app: AppHandle, status_tx: UnboundedSender<ProviderStatus>) -> Self {
        Self {
            app,
            config: YoutubeConfig::default(),
            verifier: None,
            api_client: None,
            status_tx,
        }
    }

    async fn get_provider_status(&self, user_name: Option<String>) -> ProviderStatus {
        ProviderStatus {
            key: self.key(),
            name: "Youtube".into(),
            user_name: user_name.clone(),
            logged_in: user_name.is_some(),
            bg_color: "#E62017".into(),
            account_id: "youtube".into(),
            scopes: self.get_provider_scopes().await.unwrap(),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_oauth_client(&self) -> Option<OAuth2Client> {
        if self.config.client_id.is_some() && self.config.client_secret.is_some() {
            let client = BasicClient::new(
                ClientId::new(self.config.client_id.clone().unwrap()),
                Some(ClientSecret::new(
                    self.config.client_secret.clone().unwrap(),
                )),
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
                Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
            )
            .set_redirect_uri(RedirectUrl::new(self.config.redirect_uri.to_string()).unwrap());
            return Some(client);
        }
        None
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn create_api_client(&mut self) {
        if let Some(token) = &self.config.tokens {
            let client =
                hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                    .build(
                        hyper_rustls::HttpsConnectorBuilder::new()
                            .with_native_roots()
                            .unwrap()
                            .https_or_http()
                            .enable_http1()
                            .build(),
                    );

            self.api_client = Some(google_youtube3::YouTube::new(
                client,
                token.access_token.clone(),
            ));

            let res = self.fetch_user_details().await;
            if let Ok(res) = res {
                let _ = self.status_tx.send(res).await;
            } else {
                let _ = self
                    .status_tx
                    .send(self.get_provider_status(Some("".into())).await)
                    .await;
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn refresh_login(&mut self) -> Result<()> {
        let client = self.get_oauth_client();
        if let Some(client) = client {
            self.config.tokens =
                Some(refresh_login("MoosyncYoutubeRefreshToken", client, &self.app).await?);
            self.create_api_client().await;
        }

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self, resp))]
    fn parse_playlist(&self, resp: Playlist) -> QueryablePlaylist {
        let snippet = resp.snippet.unwrap_or_default();
        let content_details = resp.content_details.unwrap_or_default();

        QueryablePlaylist {
            playlist_id: Some(format!("youtube-playlist:{}", resp.id.unwrap())),
            playlist_name: snippet.title.unwrap_or_default(),
            playlist_coverpath: snippet.thumbnails.map(|t| {
                t.maxres
                    .or(t.default)
                    .unwrap_or_default()
                    .url
                    .unwrap_or_default()
            }),
            playlist_song_count: content_details.item_count.unwrap_or_default() as f64,
            playlist_desc: snippet.description,
            playlist_path: None,
            extension: Some(self.key()),
            icon: None,
            library_item: None,
        }
    }

    #[tracing::instrument(level = "trace", skip(self, resp))]
    fn parse_channel(&self, resp: Channel) -> QueryableArtist {
        let snippet = resp.snippet.as_ref().unwrap();
        QueryableArtist {
            artist_id: Some(format!("youtube-artist:{}", resp.id.clone().unwrap())),
            artist_name: snippet.title.clone(),
            artist_coverpath: snippet.thumbnails.clone().map(|t| {
                t.maxres
                    .or(t.default)
                    .unwrap_or_default()
                    .url
                    .unwrap_or_default()
            }),
            artist_extra_info: Some(EntityInfo(
                serde_json::to_string(&YoutubeExtraInfo {
                    youtube: ArtistExtraInfo {
                        artist_id: resp.id.unwrap(),
                    },
                })
                .unwrap(),
            )),
            ..Default::default()
        }
    }

    #[tracing::instrument(level = "trace", skip(self, ids))]
    async fn fetch_song_details(&self, ids: Vec<String>) -> Result<Vec<Song>> {
        tracing::info!("Fetching song details for {:?}", ids);
        if let Some(api_client) = &self.api_client {
            let mut ret = vec![];

            for id_chunk in ids.chunks(50) {
                let mut builder = api_client
                    .videos()
                    .list(&vec!["contentDetails".into(), "snippet".into()]);
                for i in id_chunk {
                    builder = builder.add_id(i);
                }

                let (_, resp) = builder.doit().await?;
                tracing::info!("Got song response {:?}", resp);
                if let Some(videos) = resp.items {
                    for v in videos {
                        ret.push(self.parse_video_item(v));
                    }
                }
            }

            return Ok(ret);
        }

        Err("API client not initialized".into())
    }

    #[tracing::instrument(level = "trace", skip(self, resp))]
    fn parse_video_item(&self, resp: Video) -> Song {
        let snippet = resp.snippet.unwrap_or_default();
        let content_details = resp.content_details.unwrap_or_default();
        let id = resp.id;

        Song {
            song: QueryableSong {
                _id: id.clone().map(|id| format!("youtube:{}", id)),
                title: snippet.title,
                date: snippet.published_at.map(|v| v.to_string()),
                duration: content_details.duration.map(|d| {
                    core::time::Duration::from(iso8601::duration(&d).unwrap()).as_secs() as f64
                }),
                type_: SongType::YOUTUBE,
                url: id.clone(),
                song_cover_path_high: snippet
                    .thumbnails
                    .clone()
                    .map(|t| t.maxres.unwrap_or_default().url.unwrap_or_default()),
                playback_url: id,
                song_cover_path_low: snippet
                    .thumbnails
                    .map(|t| t.standard.unwrap_or_default().url.unwrap_or_default()),
                date_added: snippet.published_at.map(|v| v.timestamp_millis()),
                provider_extension: Some(self.key()),
                ..Default::default()
            },
            album: Some(QueryableAlbum {
                album_name: Some("Misc".into()),
                ..Default::default()
            }),
            artists: Some(vec![QueryableArtist {
                artist_id: snippet
                    .channel_id
                    .map(|id| format!("youtube-artist:{}", id)),
                artist_name: snippet.channel_title,
                ..Default::default()
            }]),
            genre: None,
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn fetch_user_details(&self) -> Result<ProviderStatus> {
        if let Some(api_client) = &self.api_client {
            let (_, user_info) = api_client
                .channels()
                .list(&vec!["snippet".into()])
                .mine(true)
                .max_results(1)
                .doit()
                .await?;

            let mut username = Some("".to_string());
            if let Some(items) = user_info.items {
                let channel = items.first().unwrap();
                if let Some(snippet) = &channel.snippet {
                    username = snippet.title.clone();
                }
            }
            return Ok(self.get_provider_status(username).await);
        }

        Err("API client not initialized".into())
    }

    async fn search_playlists(&self, term: &str) -> Result<Vec<QueryablePlaylist>> {
        if let Some(api_client) = &self.api_client {
            return Ok(search_and_parse!(api_client, term, "playlist", |item| {
                item.id.as_ref().and_then(|id| {
                    id.playlist_id.as_ref().map(|playlist_id| {
                        let snippet = item.snippet.as_ref().unwrap();
                        let playlist = Playlist {
                            id: Some(playlist_id.clone()),
                            snippet: Some(PlaylistSnippet {
                                description: snippet.description.clone(),
                                thumbnails: snippet.thumbnails.clone(),
                                title: snippet.title.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        };
                        self.parse_playlist(playlist)
                    })
                })
            }));
        }

        let youtube_scraper: State<YoutubeScraper> = self.app.state();
        let search_res = youtube_scraper.search_yt(term).await?;

        Ok(search_res.playlists)
    }

    async fn search_artists(&self, term: &str) -> Result<Vec<QueryableArtist>> {
        if let Some(api_client) = &self.api_client {
            return Ok(search_and_parse!(api_client, &term, "channel", |item| {
                item.id.as_ref().and_then(|id| {
                    id.channel_id.as_ref().map(|channel_id| {
                        let snippet = item.snippet.as_ref().unwrap();
                        let channel = Channel {
                            id: Some(channel_id.clone()),
                            snippet: Some(ChannelSnippet {
                                description: snippet.description.clone(),
                                thumbnails: snippet.thumbnails.clone(),
                                title: snippet.title.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        };
                        self.parse_channel(channel)
                    })
                })
            }));
        }

        let youtube_scraper: State<YoutubeScraper> = self.app.state();
        let search_res = youtube_scraper.search_yt(term).await?;

        Ok(search_res.artists)
    }

    async fn fetch_artist_content(
        &self,
        artist_id: &str,
        pagination: Pagination,
    ) -> Result<(Vec<Song>, Pagination)> {
        let artist_id = artist_id.replace("youtube-artist:", "");
        if let Some(api_client) = &self.api_client {
            let mut builder = api_client
                .channels()
                .list(&vec!["contentDetails".into()])
                .max_results(50)
                .add_id(artist_id.as_str());

            if let Some(next_page) = pagination.token.clone() {
                builder = builder.page_token(next_page.as_str());
            }

            let (_, resp) = builder.doit().await?;
            if let Some(items) = resp.items {
                if let Some(items) = items.first() {
                    if let Some(content_details) = &items.content_details {
                        if let Some(related_playlists) = &content_details.related_playlists {
                            if let Some(playlist_id) = &related_playlists.uploads {
                                return self
                                    .get_playlist_content(
                                        QueryablePlaylist {
                                            playlist_id: Some(playlist_id.clone()),
                                            ..Default::default()
                                        },
                                        pagination,
                                    )
                                    .await;
                            }
                        }
                    }
                }
            };
        }

        let youtube_scraper: State<YoutubeScraper> = self.app.state();
        let search_res = youtube_scraper.search_yt(artist_id).await?;

        Ok((search_res.songs, pagination.next_page()))
    }
}

#[async_trait]
impl GenericProvider for YoutubeProvider {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn initialize(&mut self) -> Result<()> {
        let _ = self
            .status_tx
            .send(self.get_provider_status(None).await)
            .await;

        let preferences: State<PreferenceConfig> = self.app.state();
        let youtube_config: Value = preferences
            .inner()
            .load_selective("youtube".into())
            .unwrap_or_default();

        tracing::info!("{:?}", youtube_config);
        let client_id = youtube_config.get("client_id");
        let client_secret = youtube_config.get("client_secret");

        self.config.client_id = client_id.map(|v| v.as_str().unwrap().to_string());
        self.config.client_secret = client_secret.map(|v| v.as_str().unwrap().to_string());
        self.config.redirect_uri = "https://moosync.app/youtube";
        self.config.scopes = vec!["https://www.googleapis.com/auth/youtube.readonly"];

        let res = self.refresh_login().await;
        if let Err(err) = res {
            tracing::error!("youtube refresh login err: {:?}", err);
        }

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_provider_scopes(&self) -> Result<Vec<ExtensionProviderScope>> {
        Ok(vec![
            ExtensionProviderScope::Search,
            ExtensionProviderScope::Playlists,
            ExtensionProviderScope::PlaylistSongs,
            ExtensionProviderScope::PlaybackDetails,
            ExtensionProviderScope::PlaylistFromUrl,
            ExtensionProviderScope::SearchAlbum,
            ExtensionProviderScope::SearchArtist,
            ExtensionProviderScope::Recommendations,
            ExtensionProviderScope::Accounts,
            ExtensionProviderScope::ArtistSongs,
            ExtensionProviderScope::AlbumSongs,
            ExtensionProviderScope::PlaylistSongs,
        ])
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn key(&self) -> String {
        "youtube".into()
    }

    #[tracing::instrument(level = "trace", skip(self, id))]
    fn match_id(&self, id: String) -> bool {
        id.starts_with("youtube-playlist:")
            || id.starts_with("youtube-artist:")
            || id.starts_with("youtube-album:")
            || id.starts_with("youtube:")
    }

    async fn requested_account_status(&mut self) -> Result<()> {
        // TODO: Get account status from youtube
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn login(&mut self, _: String) -> Result<String> {
        let client = self.get_oauth_client();
        if let Some(client) = client {
            let (url, verifier) = login(
                LoginArgs {
                    client_id: self.config.client_id.clone(),
                    client_secret: self.config.client_secret.clone(),
                    scopes: self.config.scopes.clone(),
                    extra_params: Some(HashMap::from([
                        ("prompt", "consent"),
                        ("access_type", "offline"),
                    ])),
                },
                client,
                &self.app,
            )?;

            self.verifier = verifier;

            let oauth_handler: State<OAuthHandler> = self.app.state();
            oauth_handler.register_oauth_path("youtubeoauthcallback".into(), self.key());

            Ok(url)
        } else {
            Err("Could not create OAuth client, client_id and secret likely not set".into())
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn signout(&mut self, _: String) -> Result<()> {
        self.api_client = None;
        self.verifier = None;
        self.config.tokens = None;

        let preferences: State<PreferenceConfig> = self.app.state();
        preferences.set_secure("MoosyncYoutubeRefreshToken".into(), None::<String>)?;

        let _ = self
            .status_tx
            .send(self.get_provider_status(None).await)
            .await;

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self, code))]
    async fn authorize(&mut self, code: String) -> Result<()> {
        self.config.tokens = Some(
            authorize(
                "MoosyncYoutubeRefreshToken",
                code,
                &mut self.verifier,
                &self.app,
            )
            .await?,
        );

        self.create_api_client().await;

        // Remove
        self.fetch_user_details().await.unwrap();
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self, pagination))]
    async fn fetch_user_playlists(
        &self,
        pagination: Pagination,
    ) -> Result<(Vec<QueryablePlaylist>, Pagination)> {
        if let Some(api_client) = &self.api_client {
            if !pagination.is_first && pagination.token.is_none() {
                return Ok((vec![], pagination));
            }

            let mut builder = api_client
                .playlists()
                .list(&vec![
                    "id".into(),
                    "contentDetails".into(),
                    "snippet".into(),
                ])
                .mine(true)
                .max_results(50);

            if let Some(next_page) = pagination.token.clone() {
                builder = builder.page_token(next_page.as_str());
            }

            let (_, resp) = builder.doit().await?;
            let ret = if let Some(items) = resp.items {
                items.into_iter().map(|p| self.parse_playlist(p)).collect()
            } else {
                vec![]
            };

            tracing::info!("got user playlists: {:?}", ret);
            return Ok((ret, pagination.next_page_wtoken(resp.next_page_token)));
        }

        Err("API client not initialized".into())
    }

    #[tracing::instrument(level = "trace", skip(self, playlist, pagination))]
    async fn get_playlist_content(
        &self,
        playlist: QueryablePlaylist,
        pagination: Pagination,
    ) -> Result<(Vec<Song>, Pagination)> {
        if playlist.playlist_id.is_none() {
            return Err("Playlist ID cannot be none".into());
        }
        let playlist_id = playlist.playlist_id.unwrap();
        let playlist_id = playlist_id
            .strip_prefix("youtube-playlist:")
            .unwrap_or(&playlist_id);
        if let Some(api_client) = &self.api_client {
            if !pagination.is_first && pagination.token.is_none() {
                return Ok((vec![], pagination));
            }

            let mut builder = api_client
                .playlist_items()
                .list(&vec!["id".into(), "snippet".into()])
                .playlist_id(playlist_id)
                .max_results(50);

            if let Some(next_page) = pagination.token.clone() {
                builder = builder.page_token(next_page.as_str());
            }

            let (_, resp) = builder.doit().await?;
            let ret = if let Some(items) = resp.items {
                self.fetch_song_details(
                    items
                        .iter()
                        .filter_map(|item| {
                            item.snippet.as_ref().and_then(|id| {
                                if let Some(video_id) = id.resource_id.as_ref() {
                                    video_id.video_id.clone()
                                } else {
                                    None
                                }
                            })
                        })
                        .collect(),
                )
                .await?
            } else {
                vec![]
            };

            return Ok((ret, pagination.next_page_wtoken(resp.next_page_token)));
        }

        if !pagination.is_first {
            return Ok((vec![], pagination.next_page()));
        }

        let youtube_scraper: State<YoutubeScraper> = self.app.state();
        let res = youtube_scraper
            .get_playlist_content(playlist_id.to_string(), pagination.clone())
            .await?;

        return Ok((res.songs, pagination.next_page()));
    }

    #[tracing::instrument(level = "trace", skip(self, song, player))]
    async fn get_playback_url(&self, song: Song, player: String) -> Result<String> {
        tracing::info!("Fetching song for {} player", player);
        if song.song.provider_extension.unwrap_or_default() != self.key()
            && player.to_lowercase() == "youtube"
        {
            let youtube_scraper: State<YoutubeScraper> = self.app.state();
            let res = youtube_scraper
                .search_yt(format!(
                    "{} - {}",
                    song.artists
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|a| a.artist_name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                    song.song.title.unwrap_or_default()
                ))
                .await?;
            if let Some(first) = res.songs.first() {
                return Ok(first.song.url.clone().unwrap());
            }
        }

        if player == "local" || player == "rodio" {
            let youtube_scraper: State<YoutubeScraper> = self.app.state();
            return youtube_scraper
                .get_video_url(song.song.url.clone().unwrap())
                .await;
        } else {
            return Ok(song.song.url.clone().unwrap());
        }
    }

    #[tracing::instrument(level = "trace", skip(self, term))]
    async fn search(&self, term: String) -> Result<SearchResult> {
        if let Some(api_client) = &self.api_client {
            let mut songs = vec![];

            let song_details = search_and_parse!(api_client, &term, "video", |item| {
                item.id.as_ref().and_then(|id| id.video_id.clone())
            });

            if !song_details.is_empty() {
                songs.extend(self.fetch_song_details(song_details).await?);
            }

            let playlists = self.search_playlists(&term).await?;
            let artists = self.search_artists(&term).await?;

            return Ok(SearchResult {
                songs,
                playlists,
                artists,
                ..Default::default()
            });
        }

        let youtube_scraper: State<YoutubeScraper> = self.app.state();
        youtube_scraper.search_yt(term).await
    }

    #[tracing::instrument(level = "trace", skip(self, url))]
    async fn match_url(&self, url: String) -> Result<bool> {
        let re = Regex::new(
            r"^((?:https?:)?\/\/)?((?:www|m|music)\.)?((?:youtube\.com|youtu.be))(\/(?:[\w-]+\?v=|embed\/|v\/)?)([\w-]+)(\S+)?$",
        ).unwrap();

        Ok(re.is_match(url.as_str()))
    }

    #[tracing::instrument(level = "trace", skip(self, url))]
    async fn playlist_from_url(&self, url: String) -> Result<QueryablePlaylist> {
        let playlist_id = Url::parse(url.as_str())
            .map_err(|_| MoosyncError::String(format!("Failed to parse URL {}", url)))?;
        let playlist_id = playlist_id.query_pairs().find(|(k, _)| k == "list");

        if playlist_id.is_none() {
            return Err("Invalid URL".into());
        }

        let playlist_id = playlist_id.unwrap().1.to_string();

        if let Some(api_client) = &self.api_client {
            let (_, playlists) = api_client
                .playlists()
                .list(&vec![
                    "id".into(),
                    "contentDetails".into(),
                    "snippet".into(),
                ])
                .add_id(playlist_id.as_str())
                .max_results(1)
                .doit()
                .await?;

            if let Some(items) = playlists.items {
                if let Some(first) = items.first() {
                    let parsed = self.parse_playlist(first.clone());
                    return Ok(parsed);
                }
            }
        }

        let youtube_scraper: State<YoutubeScraper> = self.app.state();
        let res = youtube_scraper.search_yt(playlist_id).await?;
        if let Some(first) = res.playlists.first() {
            return Ok(first.clone());
        }

        Err("Playlist not found".into())
    }

    #[tracing::instrument(level = "trace", skip(self, url))]
    async fn song_from_url(&self, url: String) -> Result<Song> {
        let parsed_url = Url::parse(url.as_str())
            .map_err(|_| MoosyncError::String(format!("Failed to parse URL {}", url)))?;
        let video_id = parsed_url.query_pairs().find(|(k, _)| k == "v");
        if video_id.is_none() {
            return Err("Invalid URL".into());
        }

        let video_id = video_id.unwrap().1.to_string();

        let res = self.fetch_song_details(vec![video_id.clone()]).await;
        if let Ok(songs) = res {
            if let Some(song) = songs.first() {
                return Ok(song.clone());
            }
        }

        let youtube_scraper: State<YoutubeScraper> = self.app.state();
        let res = youtube_scraper.get_video_by_id(video_id).await?;
        Ok(res)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_suggestions(&self) -> Result<Vec<Song>> {
        if let Some(api_client) = &self.api_client {
            let (_, resp) = api_client
                .search()
                .list(&vec!["snippet".into()])
                .video_category_id("10")
                .max_results(50)
                .add_type("video")
                .doit()
                .await?;

            if let Some(items) = resp.items {
                let ids: Vec<String> = items
                    .iter()
                    .filter_map(|item| item.id.as_ref().and_then(|id| id.video_id.clone()))
                    .collect();
                if !ids.is_empty() {
                    return self.fetch_song_details(ids).await;
                }
            }
        }

        Err("Api Client not initialized".into())
    }

    async fn get_album_content(
        &self,
        album: QueryableAlbum,
        pagination: Pagination,
    ) -> Result<(Vec<Song>, Pagination)> {
        let mut id_raw = album.album_id;
        if let Some(id) = &id_raw {
            if !self.match_id(id.clone()) {
                if let Some(album_name) = album.album_name {
                    if let Some(playlist) = self.search_playlists(&album_name).await?.first() {
                        if let Some(id) = &playlist.playlist_id {
                            id_raw = Some(id.clone());
                        } else {
                            id_raw = None;
                        }
                    } else {
                        id_raw = None;
                    }
                } else {
                    id_raw = None;
                }
            }
        }

        if let Some(id) = id_raw {
            return self
                .get_playlist_content(
                    QueryablePlaylist {
                        playlist_id: Some(id.replace("youtube-album:", "youtube-playlist:")),
                        ..Default::default()
                    },
                    pagination,
                )
                .await;
        } else {
            return Err("No album found".into());
        }
    }

    async fn get_artist_content(
        &self,
        artist: QueryableArtist,
        pagination: Pagination,
    ) -> Result<(Vec<Song>, Pagination)> {
        let mut id_raw = artist.artist_id;
        if let Some(id) = &id_raw {
            if !self.match_id(id.clone()) {
                tracing::info!("ID doesn't match, searching for new ID");
                if let Some(artist_name) = artist.artist_name {
                    if let Some(artist) = self.search_artists(&artist_name).await?.first() {
                        if let Some(id) = &artist.artist_id {
                            id_raw = Some(id.clone());
                        } else {
                            id_raw = None;
                        }
                    } else {
                        id_raw = None;
                    }
                } else {
                    id_raw = None;
                }
            }
        }

        if let Some(id) = id_raw {
            tracing::info!("Found artist id {}. Now fetching contents", id);
            return self.fetch_artist_content(id.as_str(), pagination).await;
        } else {
            return Err("No artist found".into());
        }
    }

    async fn get_lyrics(&self, _: Song) -> Result<String> {
        return Err("Not implemented".into());
    }

    async fn get_song_context_menu(&self, _: Vec<Song>) -> Result<Vec<ContextMenuReturnType>> {
        return Err("Not implemented".into());
    }

    async fn get_playlist_context_menu(
        &self,
        _: QueryablePlaylist,
    ) -> Result<Vec<ContextMenuReturnType>> {
        return Err("Not implemented".into());
    }

    async fn trigger_context_menu_action(&self, _: String) -> Result<()> {
        return Err("Not implemented".into());
    }
}
