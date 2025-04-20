use log::LevelFilter;
use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;
use simplelog::{
    ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode,
};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
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

    let config_path = std::env::args().nth(1).expect("Please specify config path");
    let text = "TEST".to_string();
    let sid = std::env::args().nth(2);

    let model = piper_rs::from_config_path(Path::new(&config_path)).unwrap();
    // Set speaker ID
    if let Some(sid) = sid {
        let sid = sid.parse::<i64>().expect("Speaker ID should be number!");
        model.set_speaker(sid);
    }
    let synth = PiperSpeechSynthesizer::new(model).unwrap();
    let mut samples: Vec<f32> = Vec::new();
    let audio = synth.synthesize_parallel(text, None).unwrap();
    for result in audio {
        samples.append(&mut result.unwrap().into_vec());
    }

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let buf = SamplesBuffer::new(1, 22050, samples);
    sink.append(buf);

    sink.sleep_until_end();



}
