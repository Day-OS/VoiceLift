use serde::Deserialize;
use serde::Serialize;
use vl_global::AudioDevices;
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
