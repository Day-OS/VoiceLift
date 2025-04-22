// Demo of client RPC with no handler which just calls methods
//
// use client_rpc_handler example to test client/server
use async_lock::Mutex;
use device_linker::DeviceLinker;
use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger,
    TerminalMode,
};
use std::sync::Arc;
mod device_linker;
mod device_manager;
mod linux_device_manager;
use crate::device_manager::DeviceManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new()
            //.add_filter_ignore("easy_pw".to_owned())
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    log::info!("Starting device manager...");

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

    log::info!("Device managers: {:?}", device_managers);
    log::info!("Device linkers: {:?}", device_linkers);

    if device_linkers.is_empty() {
        log::error!("No device linkers found.");
        return Ok(());
    }
    let linker = device_linkers[0].clone();

    let mtx_linker = linker.lock().await;

    let devices: vl_global::AudioDevices =
        mtx_linker.get_devices().await?;

    if devices.input_devices.is_empty()
        || devices.output_devices.is_empty()
    {
        log::error!("No devices found.");
        log::debug!("Devices: {:?}", devices);
        return Ok(());
    }
    let input = devices.input_devices.first().unwrap();
    let output = devices.output_devices.get(1).unwrap();

    log::info!("Linking {} to {}", output, input);

    mtx_linker
        .link_device(output.clone(), input.clone())
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // Unlink devices
    log::info!("Unlinking {} from {}", output, input);
    mtx_linker
        .unlink_device(output.clone(), input.clone())
        .await?;

    Ok(())
}
