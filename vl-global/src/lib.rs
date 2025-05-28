use serde::{Deserialize, Serialize};

pub mod vl_config;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioDevices {
    pub input_devices: Vec<String>,
    pub output_devices: Vec<String>,
}
