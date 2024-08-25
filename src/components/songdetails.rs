use leptos::{
    component, create_effect, create_rw_signal, view, AnimatedShow, CollectView, IntoView,
    RwSignal, SignalGet, SignalGetUntracked, SignalSet,
};
use serde::Serialize;
use types::{songs::Song, ui::song_details::SongDetailIcons};
use wasm_bindgen_futures::spawn_local;

use crate::{
    console_log,
    icons::{
        add_to_library_icon::AddToLibraryIcon, add_to_queue_icon::AddToQueueIcon,
        pin_icon::PinIcon, plain_play_icon::PlainPlayIcon, random_icon::RandomIcon,
        song_default_icon::SongDefaultIcon,
    },
    utils::common::{format_duration, get_high_img, invoke},
};
use std::time::Duration;

#[component()]
pub fn SongDetails<T>(
    #[prop()] selected_song: T,
    #[prop()] icons: RwSignal<SongDetailIcons>,
    #[prop(optional, default = false)] show_lyrics: bool,
) -> impl IntoView
where
    T: SignalGet<Value = Option<Song>> + Copy + 'static,
{
    let selected_title = create_rw_signal(None::<String>);
    let selected_artists = create_rw_signal(None::<String>);
    let selected_duration = create_rw_signal(None::<String>);
    let selected_cover_path = create_rw_signal("".to_string());

    let selected_lyrics = create_rw_signal(None::<String>);
    let show_default_cover_img = create_rw_signal(true);
    let show_lyrics_div = create_rw_signal(false);
    let show_lyrics_always = create_rw_signal(false);

    if show_lyrics {
        create_effect(move |_| {
            console_log!("Fetching lyrics");
            let song = selected_song.get();
            if let Some(song) = song {
                let lyrics = song.song.lyrics.clone();
                if lyrics.is_none() {
                    spawn_local(async move {
                        #[derive(Serialize)]
                        struct GetLyricsArgs {
                            id: String,
                            url: String,
                            artists: Vec<String>,
                            title: String,
                        }
                        let res = invoke(
                            "get_lyrics",
                            serde_wasm_bindgen::to_value(&GetLyricsArgs {
                                id: song.song._id.clone().unwrap_or_default(),
                                url: song.song.playback_url.clone().unwrap_or_default(),
                                artists: song
                                    .artists
                                    .clone()
                                    .unwrap_or_default()
                                    .iter()
                                    .map(|a| a.artist_name.clone().unwrap_or_default())
                                    .collect::<Vec<String>>(),
                                title: song.song.title.clone().unwrap_or_default(),
                            })
                            .unwrap(),
                        )
                        .await;
                        if let Ok(res) = res {
                            let lyrics = res.as_string().unwrap();
                            selected_lyrics.set(Some(lyrics));
                        } else {
                            console_log!("Failed to fetch lyrics: {:?}", res.unwrap_err());
                        }
                    });
                    return;
                }
                selected_lyrics.set(lyrics);
            }
        });
    }

    create_effect(move |_| {
        let selected_song = selected_song.get();

        selected_title.set(
            selected_song
                .clone()
                .map(|s| s.song.clone().title.unwrap_or_default()),
        );
        selected_artists.set(Some(
            selected_song
                .as_ref()
                .map(|s| s.clone().artists.unwrap_or_default())
                .unwrap_or_default()
                .iter()
                .map(|a| a.artist_name.clone().unwrap_or_default())
                .collect::<Vec<String>>()
                .join(", "),
        ));

        selected_duration.set(
            selected_song
                .clone()
                .map(|s| format_duration(s.song.duration.unwrap_or(-1f64))),
        );

        if let Some(selected_song) = selected_song {
            selected_cover_path.set(get_high_img(&selected_song));
            show_default_cover_img.set(false);
        } else {
            selected_cover_path.set("".to_string());
            show_default_cover_img.set(true);
        }
    });

    view! {
        <div class="container-fluid scrollable" style="max-height: 100%;">
            <div class="row no-gutters">
                <div class="col position-relative">

                    <div class="image-container w-100">
                        <div class="embed-responsive embed-responsive-1by1">
                            <div
                                class="embed-responsive-item albumart"
                                on:mouseenter=move |_| {
                                    if show_lyrics {
                                        console_log!("showing lyrics");
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
                                        view! {
                                            <img
                                                src=cover_path
                                                on:error=move |_| { show_default_cover_img.set(true) }
                                            />
                                        }
                                            .into_view()
                                    } else {
                                        view! {
                                            <SongDefaultIcon class="fade-in-image".to_string() />
                                        }
                                            .into_view()
                                    }
                                }}
                                <AnimatedShow
                                    when=show_lyrics_div
                                    show_class="fade-in-lyrics"
                                    hide_class="fade-out-lyrics"
                                    hide_delay=Duration::from_millis(200)
                                >
                                    <div class="lyrics-container">
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
                    <div class="button-group d-flex">

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
                                        },
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
                                        },
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
                                        },
                                    );
                            }
                            if let Some(random_cb) = icons.random {
                                icons_ret
                                    .push(view! { <RandomIcon on:click=move |_| random_cb() /> });
                            }
                            icons_ret.collect_view()
                        }}
                    </div>
                </div>
            </div>

        </div>
    }
}
