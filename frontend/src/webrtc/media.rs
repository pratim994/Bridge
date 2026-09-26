use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlVideoElement, MediaStream, MediaStreamConstraints, MediaStreamTrack};

pub async fn get_local_media(
    video: bool,
    audio: bool,
) -> Result<MediaStream, JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("No window"))?;

    let navigator = window.navigator();

    let media_devices = navigator.media_devices()?;

    let constraints = MediaStreamConstraints::new();

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaStatus {
    Disabled,
    Requesting,
    Active,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaUiState {
    pub status: MediaStatus,
    pub microphone_enabled: bool,
    pub camera_enabled: bool,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct MediaController {
    pub state: RwSignal<MediaUiState>,
    stream: Rc<RefCell<Option<MediaStream>>>,
}

impl MediaController {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(MediaUiState {
                status: MediaStatus::Disabled,
                microphone_enabled: false,
                camera_enabled: false,
                error: None,
            }),
            stream: Rc::new(RefCell::new(None)),
        }
    }

    pub async fn enable_local(&self, video: &HtmlVideoElement) -> Result<(), String> {
        self.state.update(|state| {
            state.status = MediaStatus::Requesting;
            state.error = None;
        });
        match get_local_media(true, true).await {
            Ok(stream) => {
                video.set_src_object(Some(&stream));
                *self.stream.borrow_mut() = Some(stream);
                self.state.update(|state| {
                    state.status = MediaStatus::Active;
                    state.microphone_enabled = true;
                    state.camera_enabled = true;
                    state.error = None;
                });
                Ok(())
            }
            Err(error) => {
                let message = js_error_message(error);
                self.state.update(|state| {
                    state.status = MediaStatus::Unavailable;
                    state.error = Some(message.clone());
                });
                Err(message)
            }
        }
    }

    pub fn toggle_microphone(&self) -> Result<(), String> {
        let enabled = !self.state.with_untracked(|state| state.microphone_enabled);
        self.set_track_enabled(false, enabled)?;
        self.state.update(|state| state.microphone_enabled = enabled);
        Ok(())
    }

    pub fn toggle_camera(&self) -> Result<(), String> {
        let enabled = !self.state.with_untracked(|state| state.camera_enabled);
        self.set_track_enabled(true, enabled)?;
        self.state.update(|state| state.camera_enabled = enabled);
        Ok(())
    }

    pub fn stop(&self, video: Option<&HtmlVideoElement>) {
        if let Some(stream) = self.stream.borrow_mut().take() {
            for value in stream.get_tracks().iter() {
                if let Ok(track) = value.dyn_into::<MediaStreamTrack>() {
                    track.stop();
                }
            }
        }
        if let Some(video) = video {
            video.set_src_object(None);
        }
        self.state.set(MediaUiState {
            status: MediaStatus::Disabled,
            microphone_enabled: false,
            camera_enabled: false,
            error: None,
        });
    }

    fn set_track_enabled(&self, camera: bool, enabled: bool) -> Result<(), String> {
        let stream = self.stream.borrow();
        let stream = stream.as_ref().ok_or_else(|| {
            "Enable local media before changing microphone or camera controls.".to_string()
        })?;
        let tracks = if camera {
            stream.get_video_tracks()
        } else {
            stream.get_audio_tracks()
        };
        if tracks.length() == 0 {
            return Err(if camera {
                "No camera track is available.".to_string()
            } else {
                "No microphone track is available.".to_string()
            });
        }
        for value in tracks.iter() {
            if let Ok(track) = value.dyn_into::<MediaStreamTrack>() {
                track.set_enabled(enabled);
            }
        }
        Ok(())
    }
}

impl Default for MediaController {
    fn default() -> Self {
        Self::new()
    }
}

fn js_error_message(error: JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Browser media permission is unavailable.".to_string())
}
