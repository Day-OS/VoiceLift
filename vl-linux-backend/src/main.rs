use busrt::broker::{Broker, ServerConfig, BROKER_NAME};
use busrt::client::AsyncClient;
use busrt::rpc::Rpc;
use busrt::rpc::RpcClient;
use busrt::QoS;
use log::LevelFilter;
use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;
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
use easy_pw::manager::{self, PipeWireManager};

static PIPEWIRE_MANAGER: OnceLock<Mutex<PipeWireManager>> =
    OnceLock::new();

#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_max_level(LevelFilter::Debug)
            .add_filter_ignore("easy_pw".to_owned())
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let _ = PIPEWIRE_MANAGER
        .set(Mutex::new(manager::PipeWireManager::default()));

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

    println!("Waiting for frames to {}", BROKER_NAME);
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

    // Initialize PipeWire
    // let manager = PipeWireManager::default();

    // // Wait for 2 seconds
    // thread::sleep(Duration::from_secs(2));

    // log::info!("Finished Loading!");
    // let mut objects = manager.objects.lock().unwrap();

    // // objects.print_nodes();

    // let microphone = objects
    //     .find_node_by_name("input.filter-chain-933-13")
    //     .unwrap()
    //     .id;
    // let source = objects.find_node_by_name("spotify").unwrap().id;

    // std::mem::drop(objects);
    // log::info!("Found nodes: {microphone} and {source}");

    // manager.link_nodes(source, microphone);
    // manager.unlink_nodes(source, microphone);

    // log::info!("Event Sent");

    // manager._main_thread.join().unwrap();

    let config_path =
        std::env::args().nth(1).expect("Please specify config path");
    let text = "TEST".to_string();
    let sid = std::env::args().nth(2);

    let model =
        piper_rs::from_config_path(Path::new(&config_path)).unwrap();
    // Set speaker ID
    if let Some(sid) = sid {
        let sid =
            sid.parse::<i64>().expect("Speaker ID should be number!");
        model.set_speaker(sid);
    }
    let synth = PiperSpeechSynthesizer::new(model).unwrap();
    let mut samples: Vec<f32> = Vec::new();
    let audio = synth.synthesize_parallel(text, None).unwrap();
    for result in audio {
        samples.append(&mut result.unwrap().into_vec());
    }

    let (_stream, handle) =
        rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let buf = SamplesBuffer::new(1, 22050, samples);
    sink.append(buf);

    sink.sleep_until_end();
}
