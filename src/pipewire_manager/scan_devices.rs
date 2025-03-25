use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use super::command::{self, ScanChannelError};
use super::device::{AudioChannel, AudioDevice, InputDevice, OutputDevice};
use crate::pipewire_manager::command::ScanDeviceError;
use crate::pipewire_manager::device::DeviceType;
use simplelog::*;


const INPUT_CLASS: &str = "Audio/Source";
const OUTPUT_CLASS: &str = "Stream/Output/Audio";


fn _scan_channel_by_name(
    channels: &mut Vec<String>,
    name: &str,
) -> Result<AudioChannel, ScanChannelError> {
    log::info!("SCANNING: {name}");
    println!("{channels:#?}");
    let mut found_channels: Vec<String> = vec![];

    for channel in channels.iter() {
        if channel.starts_with(name) {
            found_channels.push(channel.clone());
        }
    } // Remove the found channels from the list
    for channel in found_channels.iter() {
        channels.retain(|c| c != channel);
    }

    let mut mono: Option<String> = None;
    let mut front_left: Option<String> = None;
    let mut front_right: Option<String> = None;

    for channel in found_channels {
        if channel.ends_with("_MONO") {
            mono = Some(channel);
        } else if channel.ends_with("_FL") {
            front_left = Some(channel);
        } else if channel.ends_with("_FR") {
            front_right = Some(channel);
        }
    }
    if let Some(mono) = mono {
        return Ok(AudioChannel::Mono { mono });
    } else if let (Some(front_left), Some(front_right)) = (front_left, front_right) {
        return Ok(AudioChannel::Stereo {
            front_left,
            front_right,
        });
    } else {
        return Err(ScanChannelError::NoChannelsFound(name.to_owned()));
    }
}


pub fn scan_device(
    input_channels: &mut Vec<String>,
    output_channels: &mut Vec<String>,
    entry: &std::collections::HashMap<String, String>,
) -> Result<DeviceType, ScanDeviceError> {
    let media_class = entry.get("media.class").map(String::as_str).unwrap_or("");
    let binding = String::new();
    let nick = entry
        .get("node.nick")
        .unwrap_or(
            entry
                .get("node.description")
                .unwrap_or(entry.get("node.name").unwrap_or(&binding)),
        )
        .as_str();
    let name = entry.get("node.name").map(String::as_str).unwrap_or("");

    if media_class.is_empty() {
        return Err(ScanDeviceError::DeviceIgnored("Missing media class".to_string()));
    }

    if nick.is_empty() || name.is_empty() {
        return Err(ScanDeviceError::DeviceIgnored("Missing nick or name".to_string()));
    }

    if !media_class.starts_with(INPUT_CLASS) && !media_class.starts_with(OUTPUT_CLASS) {
        return Err(ScanDeviceError::DeviceIgnored(format!("Unsupported media class | {}", media_class)));
    }

    let mut device_type: Option<DeviceType> = None;

    if media_class.starts_with(INPUT_CLASS) {
        let channel = _scan_channel_by_name(input_channels, name)?;
        let device = InputDevice {
            device: AudioDevice {
                nick: nick.to_string(),
                name: name.to_string(),
                channel: channel,
            },
        };
        device_type = Some(DeviceType::Input(device));
    } else if media_class.starts_with(OUTPUT_CLASS) {
        let channel = _scan_channel_by_name(output_channels, name)?;
        let device = OutputDevice {
            device: AudioDevice {
                nick: nick.to_string(),
                name: name.to_string(),
                channel: channel,
            },
        };
        device_type = Some(DeviceType::Output(device));
    }
    device_type.ok_or(ScanDeviceError::UnexpectedError("Unexpected device type".to_owned()))
}

pub fn scan_devices() -> Result<Vec<DeviceType>, ScanDeviceError> {
    let mut entries = command::get_pw_entries()?;
    let mut filtered_enties: HashMap<String, HashMap<String, String>> = HashMap::new();

    for entry in entries.iter() {
        let name = entry.get("node.name");
        if let None = name {
            continue;
        }
        let name = name.unwrap();
        if filtered_enties.contains_key(name) {
            continue;
        }
        filtered_enties.insert(name.to_string(), entry.clone());
    }

    entries = filtered_enties.into_values().collect();

    let mut devices = Vec::new();

    let mut input_channels = command::scan_input_channels()?;
    let mut output_channels = command::scan_output_channels()?;

    for entry in entries.iter() {

        let result = scan_device(&mut input_channels, &mut output_channels, entry);
        match result {
            Ok(device) => {
                devices.push(device)
            },
            Err(e) => {
                if let ScanDeviceError::DeviceIgnored(e) = e  {
                    log::debug!("{e}");
                    continue;
                }
                log::warn!("Issue found when scanning device: {}", e);
                continue;
            },
        }
    }
    Ok(devices)
}
