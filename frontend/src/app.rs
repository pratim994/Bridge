use leptos::prelude::*;

use crate::components::table::Table;
use crate::game::{GameSession, GameState};

#[component]
pub fn App() -> impl IntoView {
    let session = GameSession::new(GameState::demo());

    view! {
        <main class="app">
            <Table session=session />
        </main>
    }
}