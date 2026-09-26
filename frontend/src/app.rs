use leptos::prelude::*;

use crate::components::table::Table;
use crate::game::{GameSession, Seat};

#[component]
pub fn App() -> impl IntoView {
    let session = GameSession::local("LOCAL-1", "You", Seat::South, 0xB12D_6E);

    view! {
        <main class="app">
            <Table session=session />
        </main>
    }
}
