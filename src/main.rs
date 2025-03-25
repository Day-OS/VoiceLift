use std::fs;

use pipewire;
use rodio::Sink;
use rodio::{
    cpal::{self, traits::HostTrait},
    Decoder, DeviceTrait, OutputStream,
};
use simplelog::*;

mod pipewire_manager;
use pipewire_manager::scan_devices::scan_devices;
fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    let crate_name = "voice_lift";
    let alsa_output_name = format!("alsa_playback.{}", crate_name);
    let selected_mic = "NoiseTorch Microphone for Trust GXT 232 Microphone";
    let host = cpal::default_host();
    let microphone: rodio::Device = host.default_input_device().unwrap().into();
    let (outstream, handle) = OutputStream::try_from_device(&microphone).unwrap();
    let audio_sink = Sink::try_new(&handle).unwrap();
    audio_sink.volume();
    audio_sink.play();
    let file = fs::File::open("/home/dani/Músicas/sem-título.mp3").unwrap();
    let source = Decoder::new(file.try_clone().unwrap()).unwrap();
    audio_sink.append(source);

    //pw_link::get_input_devices()

    let devices = scan_devices().unwrap(); // Assuming scan_devices is a function that returns Result<Vec<HashMap<String, String>>, ScanDeviceError> from pipewire_manager/command.rs
    println!("Scanned devices:");
    for device in devices {
        println!("{device:?}");
    }
    // let mut output_device: Option<OutputDevice> = None;
    // let mut input_device: Option<InputDevice> = None;

    // for device in pw_link::get_output_devices().unwrap() {
    //     if device.audio_device.name == alsa_output_name {
    //         output_device = Some(device);
    //         break;
    //     }
    // }

    // for device in pw_link::get_input_devices().unwrap() {
    //     if device.audio_device.name == selected_mic {
    //         input_device = Some(device);
    //         break;
    //     }
    // }

    // if output_device.is_none() || input_device.is_none() {
    //     panic!(
    //         "No devices were found | Output: {:?} | Input: {:?}",
    //         output_device, input_device
    //     )
    // }
    // println!("Output: {:?} | Input: {:?}", output_device, input_device);

    // output_device.unwrap().link(&input_device.unwrap());

    // while true {}
    // //audio_sink.stop();
    // //let sink = Sink::try_new(microphone);
    // println!("{:?}", microphone.name());
}
