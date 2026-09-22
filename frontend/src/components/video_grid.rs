use leptos::prelude::*;

use crate::components::video_tile::VideoTile;

#[component]
pub fn VideoGrid() -> impl IntoView {
    view! {
        <section class="video-grid">
            <h2>"Players"</h2>

            <div class="video-grid-inner">
                <VideoTile name="North".to_string()/>
                <VideoTile name="East".to_string()/>
                <VideoTile name="South".to_string()/>
                <VideoTile name="West".to_string()/>
            </div>
        </section>
    }
}