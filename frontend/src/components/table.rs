use leptos::prelude::*;

use crate::game::{GameSession, Seat};

use super::hand::Hand;
use super::trick::TrickArea;

#[component]
pub fn Table(
    session: GameSession,
) -> impl IntoView {
    view! {
        <div class="bridge-table">

            <div class="seat north">
                <PlayerLabel
                    session=session
                    seat=Seat::North
                />
            </div>

            <div class="seat west">
                <PlayerLabel
                    session=session
                    seat=Seat::West
                />
            </div>

            <div class="center-trick">
                <div class="player-label">
                    "Current Trick"
                </div>

                <TrickArea session=session />
            </div>

            <div class="seat east">
                <PlayerLabel
                    session=session
                    seat=Seat::East
                />
            </div>

            <div class="seat south">
                <div class="player-label">
                    "You"
                </div>

                <Hand session=session />
            </div>

        </div>
    }
}

#[component]
fn PlayerLabel(
    session: GameSession,
    seat: Seat,
) -> impl IntoView {
    view! {
        <div class="player-label">
            {move || {
                session
                    .state
                    .get()
                    .player_at(seat)
                    .map(|player| player.name.clone())
                    .unwrap_or_else(|| seat.label().to_string())
            }}
        </div>
    }
}


fn is_turn(seat: Seat, turn: Turn) -> bool {
    matches!(
        (seat, turn),
        (Seat::North, Turn::North)
            | (Seat::East, Turn::East)
            | (Seat::South, Turn::South)
            | (Seat::West, Turn::West)
    )
}