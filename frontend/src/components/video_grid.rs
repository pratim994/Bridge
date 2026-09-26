use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::webrtc::media::{MediaController, MediaStatus};
use crate::components::video_tile::VideoTile;

#[component]
pub fn VideoGrid() -> impl IntoView {
    let media = MediaController::new();
    let video_ref = NodeRef::<leptos::html::Video>::new();
    let enable_media = media.clone();
    let microphone = media.clone();
    let camera = media.clone();
    let stop_media = media.clone();
    let media_state = media.state;

    view! {
        <section class="video-grid">
            <h2>"Players"</h2>

            <div class="video-grid-inner">
                <div class="video-tile local-video-tile">
                    <video node_ref=video_ref autoplay=true playsinline=true muted=true></video>
                    <span class="video-player-name">"South (you)"</span>
                </div>
                <VideoTile name="West".to_string() status="Peer video unavailable".to_string()/>
                <VideoTile name="North".to_string() status="Peer video unavailable".to_string()/>
                <VideoTile name="East".to_string() status="Peer video unavailable".to_string()/>
            </div>
            <div class="media-controls">
                <button class="secondary-button" on:click=move |_| {
                    if let Some(video) = video_ref.get() {
                        let media = enable_media.clone();
                        spawn_local(async move { let _ = media.enable_local(&video).await; });
                    }
                }>{move || if media_state.get().status == MediaStatus::Active { "Refresh media" } else { "Enable camera and microphone" }}</button>
                <button class="secondary-button" disabled=move || media_state.get().status != MediaStatus::Active on:click=move |_| { let _ = microphone.toggle_microphone(); }>
                    {move || if media_state.get().microphone_enabled { "Mute microphone" } else { "Unmute microphone" }}
                </button>
                <button class="secondary-button" disabled=move || media_state.get().status != MediaStatus::Active on:click=move |_| { let _ = camera.toggle_camera(); }>
                    {move || if media_state.get().camera_enabled { "Turn camera off" } else { "Turn camera on" }}
                </button>
                <button class="secondary-button" disabled=move || media_state.get().status == MediaStatus::Disabled on:click=move |_| { stop_media.stop(video_ref.get().as_ref()); }>
                    "Stop local media"
                </button>
                {move || match media_state.get().status {
                    MediaStatus::Disabled => Some(view! { <span class="media-status">"Camera and microphone are off"</span> }.into_any()),
                    MediaStatus::Requesting => Some(view! { <span class="media-status" role="status">"Requesting camera and microphone…"</span> }.into_any()),
                    MediaStatus::Active => Some(view! { <span class="media-status">"Local media active; peers await room signaling"</span> }.into_any()),
                    MediaStatus::Unavailable => Some(view! { <span class="media-status" role="alert">{media_state.get().error.unwrap_or_else(|| "Media unavailable".to_string())}</span> }.into_any()),
                }}
            </div>
        </section>
    }
}
