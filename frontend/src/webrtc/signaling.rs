use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalMessage {
    Offer {
        from: String,
        to: String,
        sdp: String,
    },

    Answer {
        from: String,
        to: String,
        sdp: String,
    },

    IceCandidate {
        from: String,
        to: String,
        candidate: String,
    },
}