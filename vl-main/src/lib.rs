// Demo of client RPC with no handler which just calls methods
//
// use client_rpc_handler example to test client/server

use bevy::{
    color::palettes::basic::*,
    input::{gestures::RotationGesture, touch::TouchPhase},
    log::{Level, LogPlugin},
    prelude::*,
    window::{AppLifecycle, WindowMode},
    winit::WinitSettings,
};
#[cfg(target_os = "android")]
mod android;
mod base_managers;
mod ui;

// the `bevy_main` proc_macro generates the required boilerplate for Android
#[bevy_main]
/// The entry point for the application. Is `pub` so that it can be used from
/// `main.rs`.
pub fn main() {
    ui::bevy_app::run();
}
