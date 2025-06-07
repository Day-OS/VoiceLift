// Demo of client RPC with no handler which just calls methods
//
// use client_rpc_handler example to test client/server
#![allow(internal_features)]
#![feature(core_intrinsics)]

use bevy::prelude::bevy_main;
pub mod modules;
pub mod ui;

#[bevy_main]
pub fn main() {
    ui::bevy_app::run();
}
