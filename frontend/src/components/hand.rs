use leptos::prelude::*;

use crate::game::GameSession;

use super::card::CardView;

#[component]
pub fn Hand(
    session: GameSession,
) -> impl IntoView {
    let on_card_click = Callback::new(move |card| {
        session.play_card(card);
    });

    view! {
        <div class="hand">
            {move || {
                session
                    .state
                    .get()
                    .my_hand
                    .into_iter()
                    .map(|card| {
                        view! {
                            <CardView
                                card=card
                                playable=session.is_my_turn()
                                on_click=on_card_click
                            />
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}