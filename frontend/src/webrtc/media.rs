use wasm_bindgen::JsValue;
use web_sys::{MediaStream, MediaStreamConstraints};

pub async fn get_local_media(
    video: bool,
    audio: bool,
) -> Result<MediaStream, JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("No window"))?;

    let navigator = window.navigator();

    let media_devices = navigator.media_devices()?;

    let mut constraints = MediaStreamConstraints::new();

    constraints.set_video(&JsValue::from_bool(video));
    constraints.set_audio(&JsValue::from_bool(audio));

    let promise = media_devices
        .get_user_media_with_constraints(&constraints)?;

    let stream = wasm_bindgen_futures::JsFuture::from(promise)
        .await?;

    Ok(MediaStream::from(stream))
}

pub struct VideoParticipant {
    pub player_id: String,
    pub player_name: String,
    pub stream: Option<MediaStream>,
    pub muted: bool,
}