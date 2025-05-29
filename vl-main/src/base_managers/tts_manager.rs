use futures::future::BoxFuture;
use std::fmt::Debug;
use vl_global::AudioDevices;

use super::Module;

/// Responsible for managing TTS
pub trait TtsManager: Module {
    fn speak(
        &self,
        text: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn stop_speaking(
        &self,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn get_module_type(&self) -> &'static str {
        "TTS Manager"
    }
}
