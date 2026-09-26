use leptos::prelude::*;

use crate::game::GameSession;

#[component]
pub fn Scoreboard(session: GameSession) -> impl IntoView {
    view! {
        <section class="scoreboard">
            <div class="section-heading"><h2>"Contract & score"</h2></div>
            <div class="score-grid">
                <div><span>"Contract"</span><strong>{move || session.state.with(|state| state.contract.as_ref().map_or("—".to_string(), |contract| contract.label()))}</strong></div>
                <div><span>"Declarer"</span><strong>{move || session.state.with(|state| state.declarer.map_or("—".to_string(), |seat| seat.label().to_string()))}</strong></div>
                <div><span>"Tricks won"</span><strong>{move || session.state.with(|state| state.tricks_won)}</strong></div>
                <div><span>"Trick"</span><strong>{move || session.state.with(|state| format!("{} / 13", state.completed_tricks.len()))}</strong></div>
                <div><span>"N/S score"</span><strong>{move || session.state.with(|state| state.score.north_south)}</strong></div>
                <div><span>"E/W score"</span><strong>{move || session.state.with(|state| state.score.east_west)}</strong></div>
            </div>
        </section>
    }
}
