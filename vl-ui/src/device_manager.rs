use futures::future::BoxFuture;
use std::fmt::Debug;
use vl_global::AudioDevices;

pub trait DeviceManager: Debug {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>;
}
