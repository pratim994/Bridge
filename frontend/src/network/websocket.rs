use wasm_bindgen::JsValue;
use web_sys::WebSocket;

use crate::game::commands::GameCommand;

pub struct GameSocket {
    socket: WebSocket,
}

impl GameSocket {
    pub fn connect(url: &str) -> Result<Self, JsValue> {
        let socket = WebSocket::new(url)?;

        socket.set_binary_type(
            web_sys::BinaryType::Arraybuffer
        );

        Ok(Self { socket })
    }

    pub fn send(
        &self,
        command: &GameCommand,
    ) -> Result<(), JsValue> {
        let payload =
            serde_json::to_string(command)
                .map_err(|error| {
                    JsValue::from_str(
                        &error.to_string()
                    )
                })?;

        self.socket.send_with_str(&payload)
    }

    pub fn close(&self) -> Result<(), JsValue> {
        self.socket.close()
    }
}