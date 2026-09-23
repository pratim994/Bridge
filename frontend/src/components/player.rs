use leptos::prelude::*;

use crate::game::Seat;

#[component]
pub fn PlayerSeat(
    seat: Seat,
    name: String,
    #[prop(optional)] connected: bool,
) -> impl IntoView {
    let connection_class = if connected {
        "player-status connected"
    } else {
        "player-status disconnected"
    };

    view! {
        <div class=format!(
            "player-seat player-{}",
            seat.label().to_lowercase()
        )>
            <div class="player-name">
                {name}
            </div>

            <div class=connection_class>
                {if connected { "Online" } else { "Offline" }}
            </div>
        </div>
    }
}