use busrt::broker::{Broker, ServerConfig, BROKER_NAME};
use busrt::client::AsyncClient;
use busrt::rpc::Rpc;
use busrt::rpc::RpcClient;
use busrt::QoS;
use log::LevelFilter;
use piper::PiperTTSManager;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger,
    TerminalMode,
};
use std::path::Path;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;
use tokio::time::sleep;
use vl_global::vl_config::ConfigManager;
mod event_handler;
mod event_parameters;
mod events;
mod piper;
use easy_pw::manager::{self, PipeWireManager};

static PIPEWIRE_MANAGER: OnceLock<RwLock<PipeWireManager>> =
    OnceLock::new();

static PIPERTTS_MANAGER: OnceLock<Arc<RwLock<PiperTTSManager>>> =
    OnceLock::new();

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new()
            //.add_filter_ignore("easy_pw".to_owned())
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let mut piper_model_path = None;
    let mut config = ConfigManager::new().unwrap();
    config.modify_and_save(|config| {
        if config.linux.is_none() {
            panic!("The Linux config section was not found in the config file.")
        }

        let linux = config.linux.as_ref().unwrap();
        piper_model_path = Some(linux.piper_tts_model.clone());
    }).unwrap();

    let path = piper_model_path.unwrap();
    let piper_model_path = Path::new(&path);
    if !piper_model_path.exists() {
        panic!(
            "Piper model path does not exist: {piper_model_path:?}"
        )
    }

    let _ = PIPEWIRE_MANAGER
        .set(RwLock::new(manager::PipeWireManager::default()));

    let piper_tts_manager =
        piper::PiperTTSManager::new(piper_model_path, 1).unwrap();

    let lock_pipertts = Arc::new(RwLock::new(piper_tts_manager));
    _ = PIPERTTS_MANAGER.set(lock_pipertts.clone());

    // create a new broker instance
    let mut broker = Broker::new();
    broker
        .spawn_unix_server(
            "/tmp/voicelift.sock",
            ServerConfig::default(),
        )
        .await
        .unwrap();

    let mut core_client =
        broker.register_client(BROKER_NAME).await.unwrap();
    log::debug!("NAME: {}", core_client.get_name());

    // subscribe the core client to all topics to print publish frames when received
    core_client.subscribe("#", QoS::No).await.unwrap();

    // create handlers object
    let handlers = event_handler::EventHandler {};
    // create RPC
    let crpc = RpcClient::new(core_client, handlers);

    log::info!("Waiting for frames to {BROKER_NAME}");
    // set broker client, optional, allows to spawn fifo servers, the client is wrapped in
    // Arc<Mutex<_>> as it is cloned for each fifo spawned and can be got back with core_rpc_client
    // broker method
    broker.set_core_rpc_client(crpc).await;
    // test it with echo .broker .hello > /tmp/busrt.fifo
    broker.spawn_fifo("/tmp/busrt.fifo", 8192).await.unwrap();
    // this is the internal client, it will be connected forever
    while broker
        .core_rpc_client()
        .lock()
        .await
        .as_ref()
        .unwrap()
        .is_connected()
    {
        sleep(Duration::from_secs(1)).await;
    }
}
