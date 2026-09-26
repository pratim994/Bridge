use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sequence: u64,
    pub player_id: String,
    pub player_name: String,
    pub text: String,
}

pub trait ChatClient: Send {
    fn send(
        &mut self,
        player_id: &str,
        player_name: &str,
        text: &str,
    ) -> Result<ChatMessage, String>;
}

#[derive(Debug, Default)]
pub struct LocalChatClient {
    next_sequence: u64,
}

impl ChatClient for LocalChatClient {
    fn send(
        &mut self,
        player_id: &str,
        player_name: &str,
        text: &str,
    ) -> Result<ChatMessage, String> {
        let text = text.trim();
        if text.is_empty() || text.chars().count() > 300 {
            return Err("Enter a message with 1 to 300 characters.".to_string());
        }
        self.next_sequence += 1;
        Ok(ChatMessage {
            sequence: self.next_sequence,
            player_id: player_id.to_string(),
            player_name: player_name.to_string(),
            text: text.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_chat_preserves_identity_and_trims_text() {
        let mut client = LocalChatClient::default();
        let message = client.send("south", "You", "  hello  ").unwrap();
        assert_eq!(message.text, "hello");
        assert_eq!(message.player_id, "south");
        assert_eq!(message.sequence, 1);
    }

    #[test]
    fn local_chat_rejects_empty_and_overlong_messages() {
        let mut client = LocalChatClient::default();
        assert!(client.send("south", "You", "  ").is_err());
        assert!(client.send("south", "You", &"x".repeat(301)).is_err());
    }
}
