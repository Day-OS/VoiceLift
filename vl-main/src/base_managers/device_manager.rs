use futures::future::BoxFuture;
use std::fmt::Debug;
use vl_global::AudioDevices;

use super::Module;

pub trait DeviceManager: Module {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>;
    fn get_module_type(&self) -> &'static str {
        "Device Manager"
    }
}
