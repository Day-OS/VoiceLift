use futures::future::BoxFuture;
use std::fmt::Debug;
use vl_global::AudioDevices;

/// Responsible for managing TTS
pub trait TtsManager: Debug {
    fn speak(
        &self,
        text: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;
}
