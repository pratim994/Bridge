use leptos::prelude::*;

mod api;
mod app;
mod components;
mod game;
mod network;
mod webrtc;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(app::App);
}