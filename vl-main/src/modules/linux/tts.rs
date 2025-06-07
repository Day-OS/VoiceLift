use async_lock::RwLock;
use busrt::rpc::Rpc;
use busrt::{QoS, async_trait};
use std::sync::Arc;
use vl_global::vl_config::ConfigManager;
use vl_linux_backend::error::LinuxBackendError;
use vl_linux_backend::events::client::{
    self, METHOD_SPEAK, METHOD_STOP_SPEAK,
};

use crate::modules::base::tts_module::TtsModule;
use crate::modules::linux::BROKER_NAME;
use crate::modules::linux::error::LinuxModuleError;
use crate::modules::linux::linux_module::LinuxModule;

#[async_trait]
impl TtsModule for LinuxModule {
    async fn speak(
        &self,
        text: String,
        config: Arc<RwLock<ConfigManager>>,
    ) -> anyhow::Result<()> {
        let config = config.read().await;
        let config = config.read()?;
        if config.linux.is_none() {
            return Err(
                LinuxBackendError::ConfigSectionNotFound.into()
            );
        }
        let linux_config = config.linux.unwrap();

        if let Some(client) = &self._client {
            let result = client
                .call(
                    BROKER_NAME,
                    METHOD_SPEAK,
                    rmp_serde::to_vec_named(&client::RequestTTS {
                        phrase: text,
                        pitch: linux_config.pitch,
                        volume: linux_config.volume,
                    })?
                    .into(),
                    QoS::Processed,
                )
                .await
                .map_err(|e| {
                    let empty_str = "empty_data";
                    let data =
                        e.data().unwrap_or(empty_str.as_bytes());
                    String::from_utf8(data.to_vec())
                })
                .unwrap();

            let response: client::ResponseTTS =
                rmp_serde::from_slice(result.payload())?;
            // Throws error if the result is not successful
            response
                .result
                .map_err(LinuxModuleError::FailedToSpeak)?;
            Ok(())
        } else {
            Err(LinuxModuleError::BackendServiceNotStarted.into())
        }
    }

    async fn stop_speaking(&self) -> anyhow::Result<()> {
        if let Some(client) = &self._client {
            let result = client
                .call(
                    BROKER_NAME,
                    METHOD_STOP_SPEAK,
                    rmp_serde::to_vec_named(
                        &client::RequestStopTTS {},
                    )?
                    .into(),
                    QoS::Processed,
                )
                .await
                .map_err(|e| {
                    let empty_str = "empty_data";
                    let data =
                        e.data().unwrap_or(empty_str.as_bytes());
                    String::from_utf8(data.to_vec())
                })
                .unwrap();

            let response: client::ResponseStopTTS =
                rmp_serde::from_slice(result.payload())?;
            // Throws error if the result is not successful
            response
                .result
                .map_err(LinuxModuleError::FailedToSpeak)?;
            Ok(())
        } else {
            Err(LinuxModuleError::BackendServiceNotStarted.into())
        }
    }
}
