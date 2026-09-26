use leptos::prelude::*;

use crate::game::GameSession;

use super::card::CardView;

#[component]
pub fn TrickArea(session: GameSession) -> impl IntoView {
    view! {
        <div class="trick-area" aria-label="Current trick">
            {move || {
                let state = session.state.get();
                let (trick, winner, is_previous) = if state.current_trick.plays.is_empty() {
                    state.completed_tricks.last()
                        .map(|complete| (complete.trick.clone(), Some(complete.winner), true))
                        .unwrap_or((state.current_trick.clone(), None, false))
                } else {
                    (state.current_trick.clone(), None, false)
                };
                let play_count = trick.plays.len();
                let cards = trick.plays.into_iter().map(|played| {
                    let position = relative_position(state.my_seat, played.seat);
                    view! {
                        <div class=format!("trick-card {position}")>
                            <CardView card=played.card playable=false />
                            <span class="trick-player">{played.seat.short_label()}</span>
                        </div>
                    }
                }).collect_view();
                let caption = if is_previous {
                    winner.map(|seat| format!("{} won the last trick", seat.label()))
                        .unwrap_or_else(|| "Last trick".to_string())
                } else if play_count == 0 {
                    "Lead a card to start the trick".to_string()
                } else {
                    format!("{} card{} played", play_count, if play_count == 1 { "" } else { "s" })
                };
                view! {
                    <div class=if is_previous { "trick visible previous-trick" } else { "trick visible" }>
                        {cards}
                    </div>
                    <p class="trick-caption">{caption}</p>
                }
            }}
        </div>
    }
}

fn relative_position(
    local: crate::game::Seat,
    seat: crate::game::Seat,
) -> &'static str {
    match (seat.index() + 4 - local.index()) % 4 {
        0 => "south-card",
        1 => "west-card",
        2 => "north-card",
        _ => "east-card",
    }
}
