use leptos::prelude::*;

use crate::components::{
    chat::Chat,
    connection_status::ConnectionStatus,
    scoreboard::Scoreboard,
    table::BridgeTable,
    video_grid::VideoGrid,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="app">
            <header class="app-header">
                <div>
                    <h1>"BridgeRoom"</h1>
                    <span class="room-id">
                        "Room #DEMO"
                    </span>
                </div>

                <ConnectionStatus/>
            </header>

            <div class="game-layout">

                <section class="game-board">
                    <BridgeTable/>
                    <Scoreboard/>
                </section>

                <aside class="sidebar">

                    <VideoGrid/>

                    <Chat/>

                </aside>

            </div>
        </main>
    }
}