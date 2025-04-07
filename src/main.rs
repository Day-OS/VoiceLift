use log::LevelFilter;
use pipewire_manager::PipeWireManager;
use simplelog::{
    ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode,
};
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

    objects.print_nodes();

    let microphone = objects
        .find_node_by_name("input.filter-chain-924-13")
        .unwrap()
        .id;
    let source = objects.find_node_by_name("spotify").unwrap().id;

    std::mem::drop(objects);
    log::info!("Found nodes: {microphone} and {source}");

    manager.link_nodes(source, microphone);
    manager.unlink_nodes(source, microphone);


    log::info!("Event Sent");

    manager._main_thread.join().unwrap();
}
