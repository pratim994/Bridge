use leptos::prelude::*;

use crate::game::GameSession;

use super::card::CardView;

#[component]
pub fn TrickArea(
    session: GameSession,
) -> impl IntoView {
    view! {
        <div class="trick-area">
            {move || {
                session
                    .state
                    .get()
                    .current_trick
                    .into_iter()
                    .map(|played| {
                        view! {
                            <div class="trick-card">
                                <CardView
                                    card=played.card
                                    playable=false
                                />
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}