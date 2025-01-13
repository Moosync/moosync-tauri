use leptos::{component, html::Div, prelude::*, view, IntoView};
use leptos_use::use_resize_observer;
use types::{
    songs::Song,
    ui::song_details::{DefaultDetails, SongDetailIcons},
};
use wasm_bindgen_futures::spawn_local;

use crate::{
    icons::{
        add_to_library_icon::AddToLibraryIcon, add_to_queue_icon::AddToQueueIcon,
        fav_playlist_icon::FavPlaylistIcon, pin_icon::PinIcon, plain_play_icon::PlainPlayIcon,
        random_icon::RandomIcon, song_default_icon::SongDefaultIcon,
    },
    utils::{
        common::{format_duration, get_high_img},
        invoke::get_lyrics,
    },
};
use std::time::Duration;

#[tracing::instrument(
    level = "trace",
    skip(
        selected_song,
        icons,
        show_lyrics,
        default_details,
        buttons_ref,
        root_ref
    )
)]
#[component()]
pub fn SongDetails<T>(
    #[prop()] selected_song: T,
    #[prop()] icons: RwSignal<SongDetailIcons>,
    #[prop(optional, default = false)] show_lyrics: bool,
    #[prop(optional)] default_details: RwSignal<DefaultDetails>,
    #[prop(optional)] buttons_ref: Option<NodeRef<Div>>,
    #[prop(optional)] root_ref: Option<NodeRef<Div>>,
) -> impl IntoView
where
    T: Get<Value = Option<Song>> + Copy + 'static,
{
    let selected_title = RwSignal::new(default_details.get().title);
    let selected_artists = RwSignal::new(default_details.get().subtitle);
    let selected_duration = RwSignal::new(None::<String>);
    let selected_cover_path = RwSignal::new(default_details.get().icon);

    let selected_lyrics = RwSignal::new(None::<String>);
    let show_default_cover_img = RwSignal::new(true);
    let show_lyrics_div = RwSignal::new(false);
    let show_lyrics_always = RwSignal::new(false);

    let show_lyrics_old = RwSignal::new(false);

    let buttons_ref = if buttons_ref.is_some() {
        buttons_ref.unwrap()
    } else {
        NodeRef::new()
    };

    let root_ref = if root_ref.is_some() {
        root_ref.unwrap()
    } else {
        NodeRef::new()
    };

    if show_lyrics {
        use_resize_observer(document().body().unwrap(), move |entries, _| {
            if let Some(entry) = entries.first() {
                let rect = entry.content_rect();

                if rect.width() <= 800f64 {
                    show_lyrics_old.set(show_lyrics_div.get_untracked());
                    show_lyrics_always.set(true);
                    show_lyrics_div.set(true);
                } else {
                    show_lyrics_div.set(show_lyrics_old.get_untracked());
                    show_lyrics_always.set(show_lyrics_old.get_untracked());
                }
            }
        });

        Effect::new(move || {
            tracing::debug!("Fetching lyrics");
            let song = selected_song.get();
            if let Some(song) = song {
                let lyrics = song.song.lyrics.clone();
                if lyrics.is_none() {
                    spawn_local(async move {
                        let res = get_lyrics(
                            song.song._id.clone().unwrap_or_default(),
                            song.song.playback_url.clone().unwrap_or_default(),
                            song.artists
                                .clone()
                                .unwrap_or_default()
                                .iter()
                                .map(|a| a.artist_name.clone().unwrap_or_default())
                                .collect::<Vec<String>>(),
                            song.song.title.clone().unwrap_or_default(),
                        )
                        .await;
                        if let Ok(lyrics) = res {
                            selected_lyrics.set(Some(lyrics));
                        } else {
                            tracing::error!("Failed to fetch lyrics: {:?}", res.unwrap_err());
                        }
                    });
                    return;
                }
                selected_lyrics.set(lyrics);
            }
        });
    }

    Effect::new(move || {
        let selected_song = selected_song.get();
        let default_details = default_details.get();

        if let Some(selected_song) = selected_song {
            selected_title.set(selected_song.song.title.clone());
            selected_artists.set(selected_song.artists.as_ref().map(|a| {
                a.iter()
                    .map(|a| a.artist_name.clone().unwrap_or_default())
                    .collect::<Vec<String>>()
                    .join(", ")
            }));
            selected_duration.set(Some(format_duration(
                selected_song.song.duration.unwrap_or(-1f64),
            )));
            selected_cover_path.set(Some(get_high_img(&selected_song)));
            show_default_cover_img.set(false);
        } else {
            show_default_cover_img.set(default_details.icon.is_none());
            selected_cover_path.set(default_details.icon);
            selected_title.set(default_details.title);
            selected_artists.set(default_details.subtitle);
            selected_duration.set(None);
        }
    });

    view! {
        <div
            class="container-fluid scrollable song-details"
            style="max-height: 100%;"
            node_ref=root_ref
        >
            <div class="row no-gutters">
                <div class="col position-relative">

                    <div class="image-container w-100">
                        <div class="embed-responsive embed-responsive-1by1">
                            <div
                                class="embed-responsive-item albumart"
                                on:mouseenter=move |_| {
                                    if show_lyrics {
                                        tracing::debug!("showing lyrics");
                                        show_lyrics_div.set(true)
                                    }
                                }
                                on:mouseleave=move |_| {
                                    if show_lyrics && !show_lyrics_always.get_untracked() {
                                        show_lyrics_div.set(false)
                                    }
                                }
                            >

                                {move || {
                                    let cover_path = selected_cover_path.get();
                                    if !show_default_cover_img.get() {
                                        tracing::debug!("Got cover path {:?}", cover_path);
                                        if let Some(cover) = cover_path.clone() {
                                            if cover == "favorites" {
                                                return view! { <FavPlaylistIcon class="" /> }.into_any();
                                            }
                                        }
                                        view! {
                                            <img
                                                src=cover_path
                                                on:error=move |_| { show_default_cover_img.set(true) }
                                            />
                                        }
                                            .into_any()
                                    } else {
                                        view! { <SongDefaultIcon /> }.into_any()
                                    }
                                }}
                                <AnimatedShow
                                    when=show_lyrics_div
                                    show_class="fade-in-lyrics"
                                    hide_class="fade-out-lyrics"
                                    hide_delay=Duration::from_millis(200)
                                >
                                    <div class="lyrics-container">
                                        <div class="lyrics-side-decoration"></div>
                                        <div class="lyrics-background"></div>
                                        <pre>{move || selected_lyrics.get()}</pre>
                                        <PinIcon
                                            filled=show_lyrics_always
                                            on:click=move |_| {
                                                if show_lyrics_always.get() {
                                                    show_lyrics_always.set(false)
                                                } else {
                                                    show_lyrics_always.set(true)
                                                }
                                            }
                                        />
                                    </div>
                                </AnimatedShow>

                            </div>
                        </div>
                    </div>

                    <div class="song-info-container">
                        <div class="row d-flex">
                            <div class="col song-title-details text-truncate">
                                {move || selected_title.get()}

                            </div>
                        </div>

                        <div class="song-subtitle-details text-truncate">
                            {move || selected_artists.get()}

                        </div>

                        <div class="song-timestamp-details">

                            {move || selected_duration.get()}

                        </div>
                    </div>
                </div>
            </div>

            <div class="row no-gutters flex-fill mt-2">
                <div class="col">
                    <div class="button-group d-flex" node_ref=buttons_ref>

                        {move || {
                            let title = selected_title.get();
                            let icons = icons.get();
                            let mut icons_ret = vec![];
                            if let Some(play_cb) = icons.play {
                                icons_ret
                                    .push(
                                        view! {
                                            <PlainPlayIcon
                                                title=title.clone().unwrap_or_default()
                                                on:click=move |_| play_cb()
                                            />
                                        }
                                            .into_any(),
                                    );
                            }
                            if let Some(add_to_queue_cb) = icons.add_to_queue {
                                icons_ret
                                    .push(
                                        view! {
                                            <AddToQueueIcon
                                                title=title.clone().unwrap_or_default()
                                                on:click=move |_| add_to_queue_cb()
                                            />
                                        }
                                            .into_any(),
                                    );
                            }
                            if let Some(add_to_library_cb) = icons.add_to_library {
                                icons_ret
                                    .push(
                                        view! {
                                            <AddToLibraryIcon
                                                title=title.unwrap_or_default()
                                                on:click=move |_| add_to_library_cb()
                                            />
                                        }
                                            .into_any(),
                                    );
                            }
                            if let Some(random_cb) = icons.random {
                                icons_ret
                                    .push(
                                        view! { <RandomIcon on:click=move |_| random_cb() /> }
                                            .into_any(),
                                    );
                            }
                            icons_ret.collect_view()
                        }}
                    </div>
                </div>
            </div>

        </div>
    }
}
