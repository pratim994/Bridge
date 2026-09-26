use leptos::prelude::*;

use crate::game::{GameSession, Seat};

use super::card::CardBack;

#[component]
pub fn PlayerSeat(session: GameSession, seat: Seat) -> impl IntoView {
    view! {
        <div class=move || {
            let state = session.state.get();
            let active = state.current_turn.seat() == seat;
            let local = state.my_seat == seat;
            format!(
                "player-panel{}{}",
                if active { " active-turn" } else { "" },
                if local { " local-player" } else { "" },
            )
        }>
            <div class="player-name">
                {move || session.state.with(|state| {
                    state.player_at(seat)
                        .map(|player| player.name.clone())
                        .unwrap_or_else(|| seat.label().to_string())
                })}
                {move || if session.state.with(|state| state.my_seat == seat) {
                    " (You)".into_view()
                } else {
                    "".into_view()
                }}
            </div>
            <div class="player-meta">
                <span class=move || if session.state.with(|state| state.player_at(seat).is_some_and(|p| p.connected)) {
                    "player-status connected"
                } else {
                    "player-status disconnected"
                }>
                    {move || if session.state.with(|state| state.player_at(seat).is_some_and(|p| p.connected)) { "Online" } else { "Offline" }}
                </span>
                <span class="seat-name">{seat.label()}</span>
                {move || if session.state.with(|state| state.current_turn.seat() == seat) {
                    view! { <span class="turn-indicator">"To play"</span> }.into_any()
                } else {
                    ().into_any()
                }}
            </div>
            {move || if session.state.with(|state| state.my_seat != seat) {
                let count = session.state.with(|state| state.player_at(seat).map_or(0, |p| p.card_count));
                view! {
                    <div class="opponent-hand" aria-label=format!("{} has {} cards", seat.label(), count)>
                        <div class="opponent-cards">
                            {(0..count.min(4)).map(|_| view! { <CardBack/> }).collect_view()}
                        </div>
                        <span class="card-count">{count} " cards"</span>
                    </div>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
