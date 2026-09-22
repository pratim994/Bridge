use leptos::prelude::*;

use crate::game::{Player, Seat};

use super::card::CardBack;

#[component]
pub fn Opponent(
    player: Player,
    card_count: usize,
) -> impl IntoView {
    let cards = (0..card_count)
        .map(|_| {
            view! {
                <CardBack />
            }
        })
        .collect_view();

    view! {
        <div class="opponent">

            <div class="player-name">
                {player.name}
            </div>

            <div class="opponent-cards">
                {cards}
            </div>

            <div class="card-count">
                {card_count} " cards"
            </div>

        </div>
    }
}