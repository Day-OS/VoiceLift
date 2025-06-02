use serde::Deserialize;
use serde::Serialize;
use vl_global::AudioDevices;

pub const METHOD_GET_DEVICES: &str = "get_devices";
pub const METHOD_LINK_DEVICES: &str = "link_devices";
pub const METHOD_UNLINK_DEVICES: &str = "unlink_devices";
pub const METHOD_SPEAK: &str = "speak";
pub const METHOD_STOP_SPEAK: &str = "stop_speak";

// Get Devices
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDevices {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDevices {
    pub result: Result<AudioDevices, String>,
}

// Link Devices
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDeviceLinkage {
    pub target_device: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDeviceLinkage {
    pub result: Result<(), String>,
}

// Unlink Devices
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDeviceUnLinkage {
    pub target_device: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDeviceUnLinkage {
    pub result: Result<(), String>,
}

// Talk
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTTS {
    pub phrase: String,
    pub pitch: u8,
    pub volume: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTTS {
    pub result: Result<(), String>,
}

// Talk
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestStopTTS {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStopTTS {
    pub result: Result<(), String>,
}
