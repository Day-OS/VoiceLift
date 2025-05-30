use futures::future::BoxFuture;

use super::IModule;

pub const MODULE_TYPE: &str = "TTS Module";

/// Responsible for managing TTS
pub trait TtsModule: IModule {
    fn speak(
        &self,
        text: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn stop_speaking(
        &self,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn get_module_type(&self) -> &'static str {
        MODULE_TYPE
    }
}
