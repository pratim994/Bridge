use leptos::prelude::*;

#[component]
pub fn VideoTile(name: String, status: String) -> impl IntoView {
    let label = format!("{} video unavailable", name);
    view! {
        <div class="video-tile remote-video-tile" aria-label=label>
            <div class="video-unavailable" aria-hidden="true">"○"</div>
            <span class="video-player-name">
                {name}
            </span>
            <span class="video-status">{status}</span>
        </div>
    }
}
