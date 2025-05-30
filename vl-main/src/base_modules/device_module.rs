use futures::future::BoxFuture;
use vl_global::AudioDevices;

use super::IModule;

fn is_capable_of_linking_error(capacity: bool) -> String {
    if capacity {
        return "This method was not implemented, even though the module is capable of linking devices. Please implement this module properly.".to_string();
    }
    "This module is not capable of linking devices. Please check your code.".to_string()
}
pub const MODULE_TYPE: &str = "Device Module";
pub trait DeviceModule: IModule {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>;
    fn get_module_type(&self) -> &'static str {
        MODULE_TYPE
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
