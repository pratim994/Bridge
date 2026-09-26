#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Local,
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}
