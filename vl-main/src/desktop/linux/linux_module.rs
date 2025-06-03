use crate::base_modules::tts_module::TtsModule;
use crate::base_modules::{IModule, device_module::DeviceModule};
use async_lock::RwLock;
use bevy::image::Volume;
use busrt::ipc::{Client, Config};
use busrt::rpc::{Rpc, RpcClient};
use busrt::{QoS, async_trait};
use futures::future::BoxFuture;
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;
use vl_global::audio_devices::AudioDevices;
use vl_global::vl_config::{ConfigError, ConfigManager};
use vl_linux_backend::error::LinuxBackendError;
use vl_linux_backend::event_parameters::{
    self, METHOD_GET_DEVICES, METHOD_LINK_DEVICES, METHOD_SPEAK,
    METHOD_STOP_SPEAK, METHOD_UNLINK_DEVICES,
};
const BROKER_NAME: &str = ".broker";

#[derive(Error, Debug)]
enum LinuxModuleError {
    #[error(
        "Failed to connect into the Linux Backend Socket. Reason: {0}"
    )]
    FailedToConnectIntoSocket(String),
    #[error("Linux Backend Service was not started")]
    BackendServiceNotStarted,
    #[error("Failed to get devices: {0}")]
    FailedToGetDevices(String),
    #[error("Failed to link: {0}")]
    FailedToLink(String),
    #[error("Failed to unlink: {0}")]
    FailedToUnlink(String),
    #[error("Failed to initialize speaking: {0}")]
    FailedToSpeak(String),
}

/// Resposible for linking devices in Linux with the help of a backend (vl-linux-backend)
pub struct LinuxModule {
    _client: Option<RpcClient>,
}

impl LinuxModule {
    async fn new_client()
    -> Result<RpcClient, Box<dyn std::error::Error>> {
        let name = "voice-lift-device.client";
        // create a new client instance
        let config = Config::new("/tmp/voicelift.sock", name);
        let client = Client::connect(&config).await.map_err(|e| {
            LinuxModuleError::FailedToConnectIntoSocket(e.to_string())
        })?;
        // create RPC with no handlers
        Ok(RpcClient::new0(client))
    }
    pub async fn new() -> Self {
        Self { _client: None }
    }
}
impl IModule for LinuxModule {
    fn is_started(&self) -> bool {
        self._client.is_some()
    }

    fn start(
        &mut self,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        Box::pin(async move {
            self._client = Some(Self::new_client().await?);
            Ok(())
        })
    }
}

#[async_trait]
impl DeviceModule for LinuxModule {
    async fn get_devices(&self) -> anyhow::Result<AudioDevices> {
        // call the method with no confirm
        if let Some(client) = &self._client {
            let result = client
                .call(
                    BROKER_NAME,
                    METHOD_GET_DEVICES,
                    rmp_serde::to_vec_named(
                        &event_parameters::RequestDevices {},
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

            let devices: event_parameters::ResponseDevices =
                rmp_serde::from_slice(result.payload())?;

            return Ok(devices.result.map_err(|e| {
                LinuxModuleError::FailedToGetDevices(e)
            })?);
        }
        Err(LinuxModuleError::BackendServiceNotStarted.into())
    }

    fn is_capable_of_linking(&self) -> bool {
        true
    }

    async fn link_device(
        &self,
        input_device: String,
    ) -> anyhow::Result<()> {
        if let Some(client) = &self._client {
            let result = client
                .call(
                    BROKER_NAME,
                    METHOD_LINK_DEVICES,
                    rmp_serde::to_vec_named(
                        &event_parameters::RequestDeviceLinkage {
                            target_device: input_device,
                        },
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

            let response: event_parameters::ResponseDeviceLinkage =
                rmp_serde::from_slice(result.payload())?;
            // Throws error if the result is not successful
            response
                .result
                .map_err(LinuxModuleError::FailedToLink)?;
            Ok(())
        } else {
            Err(LinuxModuleError::BackendServiceNotStarted.into())
        }
    }

    async fn unlink_device(
        &self,
        input_device: String,
    ) -> anyhow::Result<()> {
        // call the method with no confirm
        if let Some(client) = &self._client {
            let result = client
                .call(
                    BROKER_NAME,
                    METHOD_UNLINK_DEVICES,
                    rmp_serde::to_vec_named(
                        &event_parameters::RequestDeviceUnLinkage {
                            target_device: input_device,
                        },
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

            let response: event_parameters::ResponseDeviceUnLinkage =
                rmp_serde::from_slice(result.payload())?;
            // Throws error if the result is not successful
            response
                .result
                .map_err(LinuxModuleError::FailedToUnlink)?;
            Ok(())
        } else {
            Err(LinuxModuleError::BackendServiceNotStarted.into())
        }
    }
}

impl Debug for LinuxModule {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("LinuxDeviceManager").finish()
    }
}

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
                    rmp_serde::to_vec_named(
                        &event_parameters::RequestTTS {
                            phrase: text,
                            pitch: linux_config.pitch,
                            volume: linux_config.volume,
                        },
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

            let response: event_parameters::ResponseTTS =
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
                        &event_parameters::RequestStopTTS {},
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

            let response: event_parameters::ResponseStopTTS =
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
