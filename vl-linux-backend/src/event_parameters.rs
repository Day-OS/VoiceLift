use serde::Deserialize;
use serde::Serialize;

// Get Devices
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDevices {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDevices {
    pub input_devices: Vec<String>,
    pub output_devices: Vec<String>,
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
