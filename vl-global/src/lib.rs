use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioDevices {
    pub input_devices: Vec<String>,
    pub output_devices: Vec<String>,
}
