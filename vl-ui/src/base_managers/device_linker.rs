use futures::future::BoxFuture;

use super::device_manager::DeviceManager;
pub trait DeviceLinker: DeviceManager {
    fn link_device(
        &self,
        output_device: String,
        input_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn unlink_device(
        &self,
        output_device: String,
        input_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;
}
