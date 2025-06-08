// Not being used ATM
#![allow(internal_features)]
#![feature(core_intrinsics)]

use bevy::prelude::*;
pub mod events;
pub mod manager;
pub mod modules;
pub mod ui;

#[bevy_main]
pub fn main() {
    ui::bevy_app::run();
}
