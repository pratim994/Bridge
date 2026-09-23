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