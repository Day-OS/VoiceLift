// Demo of client RPC with no handler which just calls methods
//
// use client_rpc_handler example to test client/server
#![feature(core_intrinsics)]
use bevy::prelude::bevy_main;
use futures::executor;
pub mod base_managers;
pub mod desktop;
pub mod ui;

#[bevy_main]
pub fn main() {
    ui::bevy_app::run();
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     CombinedLogger::init(vec![TermLogger::new(
//         LevelFilter::Debug,
//         ConfigBuilder::new()
//             //.add_filter_ignore("easy_pw".to_owned())
//             .build(),
//         TerminalMode::Mixed,
//         ColorChoice::Auto,
//     )])
//     .unwrap();

//     log::info!("Starting device manager...");

//     let mut device_linkers: Vec<Arc<Mutex<dyn DeviceLinker>>> =
//         vec![];
//     let mut device_managers: Vec<Arc<Mutex<dyn DeviceManager>>> =
//         vec![];
//     let mut tts_managers: Vec<Arc<Mutex<dyn TtsManager>>> = vec![];

//     #[cfg(target_os = "linux")]
//     {
//         let linux_linker = Arc::new(Mutex::new(
//             linux_device_manager::LinuxDeviceManager::new().await,
//         ));
//         device_linkers.push(linux_linker.clone());
//         device_managers.push(linux_linker);
//         let linux_tts = Arc::new(Mutex::new(
//             linux_tts_manager::LinuxTtsManager::new().await,
//         ));
//         tts_managers.push(linux_tts);
//     }

//     log::info!("Device managers: {:?}", device_managers);
//     log::info!("Device linkers: {:?}", device_linkers);

//     if device_linkers.is_empty() {
//         log::error!("No device linkers found.");
//         return Ok(());
//     }
//     let linker = device_linkers[0].clone();
//     let tts = tts_managers[0].clone();

//     let mtx_linker = linker.lock().await;
//     let mtx_tts = tts.lock().await;
//     let devices: vl_global::AudioDevices =
//         mtx_linker.get_devices().await?;

//     for device in devices.output_devices.iter() {
//         log::info!("Found output device: {}", device);
//     }

//     if devices.input_devices.is_empty()
//         || devices.output_devices.is_empty()
//     {
//         log::error!("No devices found.");
//         log::debug!("Devices: {:?}", devices);
//         return Ok(());
//     }
//     let input = "Chromium input".to_owned();
//     //    let input = devices.input_devices.first().unwrap();

//     log::info!("Linking TTS OUTPUT to {}", input);

//     mtx_linker.link_device(input.clone()).await?;

//     mtx_tts.speak("Olá Mundo! :)".to_string()).await?;

//     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     // Unlink devices
//     log::info!("Unlinking TTS OUTPUT to {}", input);
//     mtx_linker.unlink_device(input.clone()).await?;
//     log::info!("Finished!");

//     Ok(())
// }
