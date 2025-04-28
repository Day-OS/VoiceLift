use futures::future::BoxFuture;
use std::fmt::Debug;
use vl_global::AudioDevices;

/// Responsible for managing TTS
pub trait TtsManager: Debug {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>;
}
