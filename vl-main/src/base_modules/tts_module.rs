use std::sync::Arc;

use async_lock::RwLock;
use busrt::async_trait;
use vl_global::vl_config::ConfigManager;

use super::IModule;

pub const MODULE_TYPE: &str = "TTS Module";

/// Responsible for managing TTS
#[async_trait]
pub trait TtsModule: IModule {
    async fn speak(
        &self,
        text: String,
        config: Arc<RwLock<ConfigManager>>,
    ) -> anyhow::Result<()>;

    async fn stop_speaking(&self) -> anyhow::Result<()>;

    fn get_module_type(&self) -> &'static str {
        MODULE_TYPE
    }
}
