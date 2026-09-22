use leptos::prelude::*;

use crate::components::card::CardView;
use crate::game::Card;

#[component]
pub fn Hand(
    cards: Vec<Card>,
) -> impl IntoView {
    view! {
        <div class="player-hand">
            {
                cards
                    .into_iter()
                    .map(|card| {
                        view! {
                            <CardView
                                card=card
                                playable=true
                            />
                        }
                    })
                    .collect_view()
            }
        </div>
    }
}