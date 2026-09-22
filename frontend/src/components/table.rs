use leptos::prelude::*;

use crate::game::{GameState, Seat};

use super::hand::Hand;
use super::trick::TrickArea;

#[component]
pub fn Table(state: GameState) -> impl IntoView {
    let north_player = state
        .players
        .iter()
        .find(|p| p.seat == Seat::North)
        .cloned();

    let west_player = state
        .players
        .iter()
        .find(|p| p.seat == Seat::West)
        .cloned();

    let east_player = state
        .players
        .iter()
        .find(|p| p.seat == Seat::East)
        .cloned();

    view! {
        <div class="bridge-table">

            <div class="seat north">
                {
                    match north_player {
                        Some(player) => view! {
                            <div class="player-label">
                                {player.name}
                            </div>
                        }.into_any(),
                        None => view! {
                            <div class="player-label">
                                "North"
                            </div>
                        }.into_any(),
                    }
                }
            </div>

            <div class="seat west">
                {
                    match west_player {
                        Some(player) => view! {
                            <div class="player-label">
                                {player.name}
                            </div>
                        }.into_any(),
                        None => view! {
                            <div class="player-label">
                                "West"
                            </div>
                        }.into_any(),
                    }
                }
            </div>

            <div class="center-trick">
                <div class="player-label">
                    "Current Trick"
                </div>

                <TrickArea />
            </div>

            <div class="seat east">
                {
                    match east_player {
                        Some(player) => view! {
                            <div class="player-label">
                                {player.name}
                            </div>
                        }.into_any(),
                        None => view! {
                            <div class="player-label">
                                "East"
                            </div>
                        }.into_any(),
                    }
                }
            </div>

            <div class="seat south">
                <div class="player-label">
                    "You"
                </div>

                <Hand cards=state.my_hand />
            </div>

        </div>
    }
}
