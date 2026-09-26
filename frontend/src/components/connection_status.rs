use leptos::prelude::*;

use crate::network::reconnect::ConnectionState;

#[component]
pub fn ConnectionStatus(
    #[prop(default = ConnectionState::Connected)] state: ConnectionState,
    #[prop(default = false)] local: bool,
) -> impl IntoView {
    let (label, class_name) = if local {
        ("Local room", "connection local")
    } else {
        match state {
            ConnectionState::Local => ("Local room", "connection local"),
            ConnectionState::Disconnected => {
                ("Disconnected", "connection disconnected")
            }
            ConnectionState::Connecting => {
                ("Connecting...", "connection connecting")
            }
            ConnectionState::Connected => ("Connected", "connection connected"),
            ConnectionState::Reconnecting => {
                ("Reconnecting...", "connection reconnecting")
            }
        }
    };

    view! {
        <div class=class_name>
            <span class="connection-dot" aria-hidden="true"></span>
            <span>{label}</span>
        </div>
    }
}
