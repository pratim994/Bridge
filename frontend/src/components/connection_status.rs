use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Reconnecting,
    Disconnected,
}

#[component]
pub fn ConnectionStatus() -> impl IntoView {
    let state = ConnectionState::Connected;

    let label = match state {
        ConnectionState::Connecting => "Connecting",
        ConnectionState::Connected => "Connected",
        ConnectionState::Reconnecting => "Reconnecting",
        ConnectionState::Disconnected => "Disconnected",
    };

    view! {
        <div class="connection-status">
            <span class="status-dot"></span>
            <span>{label}</span>
        </div>
    }
}