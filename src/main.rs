use libspa::utils::dict::DictRef;
use log::LevelFilter;
use pipewire as pw;
use pipewire::registry::GlobalObject;
use pipewire_manager::PipeWireManager;
use simplelog::{
    ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod pipewire_manager;

fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
    // Initialize PipeWire
    let manager = PipeWireManager::default();

    // Wait for 2 seconds
    thread::sleep(Duration::from_secs(2));

    log::info!("Finished Loading!");
    let mut objects = manager.objects.lock().unwrap();
    objects.nodes.iter().for_each(|node| {
        log::info!("Node ID: {}, Node Name: {}", node.id, node.name);
    });
    let microphone =  objects.find_node_by_name("NoiseTorch Microphone for Trust GXT 232 Microphone").cloned();
    let source = objects.find_node_by_name("Firefox").cloned();


    if microphone.is_none() || source.is_none() {
        log::error!("Microphone or Source not found!");
        return;
    }

    let mut microphone = microphone.unwrap();
    let mut source = source.unwrap();

    log::info!("Microphone: {:?}", microphone.name);
    log::info!("Source: {:?}", source.name);

    source.link_device(microphone);


    manager._main_thread.join().unwrap();
}
