use leptos::prelude::*;

#[component]
pub fn VideoTile(
    name: String,
) -> impl IntoView {
    view! {
        <div class="video-tile">
            <video
                autoplay=true
                playsinline=true
            ></video>

            <span class="video-player-name">
                {name}
            </span>
        </div>
    }
}