use busrt::async_trait;
use vl_global::audio_devices::AudioDevices;

use crate::modules::base::i_module::IModule;

fn is_capable_of_linking_error(capacity: bool) -> String {
    if capacity {
        return "This method was not implemented, even though the module is capable of linking devices. Please implement this module properly.".to_string();
    }
    "This module is not capable of linking devices. Please check your code.".to_string()
}
pub const MODULE_TYPE: &str = "Device Module";

#[async_trait]
pub trait DeviceModule: IModule {
    async fn get_devices(&self) -> anyhow::Result<AudioDevices>;

    fn get_module_type(&self) -> &'static str {
        MODULE_TYPE
    }
    fn is_capable_of_linking(&self) -> bool {
        false
    }

    async fn link_device(
        &self,
        _target_device: String,
    ) -> anyhow::Result<()> {
        panic!(
            "{}",
            is_capable_of_linking_error(self.is_capable_of_linking())
        )
    }

    async fn unlink_device(
        &self,
        _target_device: String,
    ) -> anyhow::Result<()> {
        panic!(
            "{}",
            is_capable_of_linking_error(self.is_capable_of_linking())
        )
    }
}
