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
    pub first_device: String,
    pub second_device: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDeviceLinkage {
    pub result: Result<(), String>,
}
