use leptos::prelude::*;

use crate::game::Card;

use super::card::CardView;

#[component]
pub fn Hand(
    cards: Vec<Card>,
) -> impl IntoView {
    view! {
        <div class="hand">
            {cards
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