use leptos::prelude::*;

use crate::game::Card;

#[component]
pub fn CardView(
    card: Card,
    #[prop(optional)] playable: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let class_name = if playable {
        "playing-card playable"
    } else {
        "playing-card"
    };

    view! {
        <button
            class=class_name
            disabled=!playable
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(());
                }
            }
        >
            <img
                src=card.asset_path()
                alt=format!(
                    "{:?} of {:?}",
                    card.rank,
                    card.suit
                )
                draggable="false"
            />
        </button>
    }
}