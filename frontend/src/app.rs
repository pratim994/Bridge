use leptos::prelude::*;

use crate::game::GameState;
use crate::components::table::Table;

#[component]
pub fn App() -> impl IntoView {
    let game = GameState::demo();

    view! {
        <main class="app">
            <Table state=game />
        </main>
    }
}