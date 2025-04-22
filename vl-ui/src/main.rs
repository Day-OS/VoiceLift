// Demo of client RPC with no handler which just calls methods
//
// use client_rpc_handler example to test client/server
use busrt::ipc::{Client, Config};
use busrt::rpc::{Rpc, RpcClient};
use busrt::{empty_payload, QoS};
use device_linker::DeviceLinker;
use log::LevelFilter;
use serde::Deserialize;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger,
    TerminalMode,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use vl_linux_backend::event_parameters;
mod device_linker;
mod device_manager;
mod linux_device_manager;
use crate::device_manager::DeviceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_max_level(LevelFilter::Debug)
            //.add_filter_ignore("easy_pw".to_owned())
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let mut device_linkers: Vec<Arc<Mutex<dyn DeviceLinker>>> =
        vec![];
    let mut device_managers: Vec<Arc<Mutex<dyn DeviceManager>>> =
        vec![];

    #[cfg(target_os = "linux")]
    {
        let linux_linker = Arc::new(Mutex::new(
            linux_device_manager::LinuxDeviceManager::new().await,
        ));
        device_linkers.push(linux_linker.clone());
        device_managers.push(linux_linker);
    }

    if device_linkers.is_empty() {
        log::error!("No device linkers found.");
        return Ok(());
    }
    let linker = device_linkers[0].clone();
    let mtx_linker = linker.lock().unwrap();
    let devices: vl_global::AudioDevices =
        mtx_linker.get_devices().await?; // Assuming get_audio_devices is implemented in LinuxDeviceManager; adjust as necessary.
    println!("{:?}", devices);
    Ok(())
}
