use leptos::prelude::*;

use crate::game::Card;

#[component]
pub fn CardView(
    card: Card,
    #[prop(optional)] playable: bool,
    #[prop(default = true)] legal: bool,
    #[prop(optional)] on_click: Option<Callback<Card>>,
) -> impl IntoView {
    let class_name = if !legal {
        "playing-card illegal"
    } else if playable {
        "playing-card playable legal"
    } else {
        "playing-card unavailable legal"
    };

    view! {
        <button
            class=class_name
            disabled=!playable
            aria-label=format!("{} of {}", card.rank.filename(), card.suit.filename())
            on:click=move |_| {
                if let Some(callback) = on_click {
                    callback.run(card);
                }
            }
        >
            <img
                src=card.asset_path()
                alt=format!(
                    "{}{}",
                    card.rank.filename(),
                    card.suit.symbol()
                )
                draggable="false"
            />
        </button>
    }
}

#[component]
pub fn CardBack() -> impl IntoView {
    view! {
        <div class="card-back" aria-label="Hidden card">
            <img
                src="/components/cards/card_back.png"
                alt="Card back"
                draggable="false"
            />
        </div>
    }
}
