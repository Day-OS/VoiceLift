use futures::future::BoxFuture;

use super::device_manager::DeviceManager;
pub trait DeviceLinker: DeviceManager {
    fn link_device(
        &self,
        target_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn unlink_device(
        &self,
        target_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn get_module_type(&self) -> &'static str {
        "Device Linker Manager"
    }
}
