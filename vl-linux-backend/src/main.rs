use busrt::broker::{Broker, ServerConfig, BROKER_NAME};
use busrt::client::AsyncClient;
use busrt::rpc::Rpc;
use busrt::rpc::RpcClient;
use busrt::QoS;
use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, TermLogger,
    TerminalMode,
};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tokio::time::sleep;
mod event_handler;
mod event_parameters;
mod events;
mod piper;
use easy_pw::manager::{self, PipeWireManager};

static PIPEWIRE_MANAGER: OnceLock<Mutex<PipeWireManager>> =
    OnceLock::new();

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

    let _ = PIPEWIRE_MANAGER
        .set(Mutex::new(manager::PipeWireManager::default()));

    let piper_tts_manager = piper::PiperTTSManager::new(
        Path::new(
            //"/usr/share/piper-voices/en/en_US/glados/high/en_us-glados-high.onnx.json",
            "/usr/share/piper-voices/pt/pt_BR/droidela-v2/medium/droidela-v2.onnx.json",
        ),
        1,
    ).unwrap();

    piper_tts_manager
        .speak("Olá mundo".to_string(), 48, 128)
        .unwrap();

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

    log::info!("Waiting for frames to {}", BROKER_NAME);
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
