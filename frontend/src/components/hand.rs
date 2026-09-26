use leptos::prelude::*;

use crate::game::GameSession;

use super::card::CardView;

#[component]
pub fn Hand(session: GameSession) -> impl IntoView {
    let click_session = session.clone();
    let on_card_click = Callback::new(move |card| {
        let _ = click_session.play_card(card);
    });

    view! {
        <section class="local-hand" aria-label="Your hand">
            <div class="hand-heading">
                <span>"Your hand"</span>
                <span class="hand-count">
                    {move || session.state.with(|state| state.my_hand.len())}
                    " cards"
                </span>
            </div>
            <div class="hand">
            {move || {
                let state = session.state.get();
                state.my_hand.cards().iter().copied().map(|card| {
                        let legal = state.can_follow_suit(card);
                        let playable = state.can_play_card(card);
                        view! {
                            <CardView
                                card=card
                                legal=legal
                                playable=playable
                                on_click=on_card_click
                            />
                        }
                    })
                    .collect_view()
            }}
            </div>
            {move || if !session.state.with(|state| state.is_my_turn()) {
                view! { <p class="hand-hint">"Waiting for your turn"</p> }.into_view()
            } else {
                view! { <p class="hand-hint">"Choose a legal card to play"</p> }.into_view()
            }}
        </section>
    }
}
