use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SignalMessage {
    Offer {
        room_id: String,
        from: String,
        to: String,
        sdp: String,
    },

    Answer {
        room_id: String,
        from: String,
        to: String,
        sdp: String,
    },

    IceCandidate {
        room_id: String,
        from: String,
        to: String,
        candidate: String,
    },
}

pub trait SignalingClient: Send {
    fn send(&mut self, message: SignalMessage) -> Result<(), String>;
}

#[derive(Debug, Default)]
pub struct LocalSignalingClient {
    queued: Vec<SignalMessage>,
}

impl LocalSignalingClient {
    pub fn take_queued(&mut self) -> Vec<SignalMessage> {
        std::mem::take(&mut self.queued)
    }
}

impl SignalingClient for LocalSignalingClient {
    fn send(&mut self, message: SignalMessage) -> Result<(), String> {
        self.queued.push(message);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signaling_message_round_trips_without_transport_assumptions() {
        let message = SignalMessage::Offer {
            room_id: "room".into(),
            from: "north".into(),
            to: "south".into(),
            sdp: "opaque-sdp".into(),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        let decoded: SignalMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(decoded, message);
    }
}
