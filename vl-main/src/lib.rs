// Not being used ATM
#![feature(core_intrinsics)]

use bevy::prelude::*;
pub mod base_modules;
pub mod desktop;
pub mod ui;

#[bevy_main]
pub fn main() {
    ui::bevy_app::run();
}
