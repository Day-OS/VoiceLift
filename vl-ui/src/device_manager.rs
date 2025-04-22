use futures::future::BoxFuture;
use vl_global::AudioDevices;

pub trait DeviceManager {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>;
}
