use crate::game::commands::GameCommand;
use crate::network::protocol::ServerMessage;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use web_sys::MessageEvent;
use web_sys::WebSocket;
pub struct GameSocket {
    socket: WebSocket,
}

impl GameSocket {
    pub fn connect(url: &str) -> Result<Self, JsValue> {
        let socket = WebSocket::new(url)?;

        socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

        Ok(Self { socket })
    }

    pub fn send(&self, command: &GameCommand) -> Result<(), JsValue> {
        let payload = serde_json::to_string(command)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;

        self.socket.send_with_str(&payload)
    }

    pub fn close(&self) -> Result<(), JsValue> {
        self.socket.close()
    }

    pub fn on_message(&self, callback: impl Fn(ServerMessage) + 'static) {
        let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                match serde_json::from_str::<ServerMessage>(&text) {
                    Ok(message) => callback(message),
                    Err(error) => {
                        web_sys::console::error_1(
                            &format!("Invalid server message: {error}").into(),
                        );
                    }
                }
            }
        })
            as Box<dyn FnMut(MessageEvent)>);
        self.socket
            .set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }

    pub fn on_open(&self, callback: impl Fn() + 'static) {
        let callback =
            Closure::wrap(Box::new(move || callback()) as Box<dyn FnMut()>);
        self.socket
            .set_onopen(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }

    pub fn on_close(&self, callback: impl Fn() + 'static) {
        let callback =
            Closure::wrap(Box::new(move || callback()) as Box<dyn FnMut()>);
        self.socket
            .set_onclose(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }
}
