use leptos::prelude::*;

use crate::network::reconnect::ConnectionState;

#[component]
pub fn ConnectionStatus(
    #[prop(default = ConnectionState::Connected)]
    state: ConnectionState,
) -> impl IntoView {
    let (label, class_name) = match state {
        ConnectionState::Disconnected => {
            ("Disconnected", "connection disconnected")
        }
        ConnectionState::Connecting => {
            ("Connecting...", "connection connecting")
        }
        ConnectionState::Connected => {
            ("Connected", "connection connected")
        }
        ConnectionState::Reconnecting => {
            ("Reconnecting...", "connection reconnecting")
        }
    };

    view! {
        <div class=class_name>
            <span class="connection-dot"></span>
            <span>{label}</span>
        </div>
    }
}