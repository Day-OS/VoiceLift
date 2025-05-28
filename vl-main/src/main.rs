// Demo of client RPC with no handler which just calls methods
//
// use client_rpc_handler example to test client/server
#![feature(core_intrinsics)]
use bevy::prelude::bevy_main;
pub mod base_managers;
pub mod desktop;
pub mod ui;

#[bevy_main]
pub fn main() {
    ui::bevy_app::run();
}
