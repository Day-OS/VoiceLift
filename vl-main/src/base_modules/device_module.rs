use futures::future::BoxFuture;
use std::fmt::Debug;
use vl_global::AudioDevices;

use super::Module;

fn is_capable_of_linking_error(capacity: bool) -> String {
    if capacity {
        return "This method was not implemented, even though the module is capable of linking devices. Please implement this module properly.".to_string();
    }
    return "This module is not capable of linking devices. Please check your code.".to_string();
}

pub trait DeviceModule: Module {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>;
    fn get_module_type(&self) -> &'static str {
        "Device Module"
    }
    fn is_capable_of_linking(&self) -> bool {
        false
    }

    fn link_device(
        &self,
        target_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        panic!(
            "{}",
            is_capable_of_linking_error(self.is_capable_of_linking())
        )
    }

    fn unlink_device(
        &self,
        target_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        panic!(
            "{}",
            is_capable_of_linking_error(self.is_capable_of_linking())
        )
    }
}
