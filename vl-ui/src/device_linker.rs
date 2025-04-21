use crate::device_manager::DeviceManager;
pub trait DeviceLinker: DeviceManager {
    fn link_device(
        &self,
        output_device: String,
        input_device: String,
    ) -> bool;
}
