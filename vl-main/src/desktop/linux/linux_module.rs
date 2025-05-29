use crate::base_modules::tts_module::TtsModule;
use crate::base_modules::{Module, device_module::DeviceModule};
use busrt::QoS;
use busrt::ipc::{Client, Config};
use busrt::rpc::{Rpc, RpcClient};
use futures::future::BoxFuture;
use std::fmt::Debug;
use thiserror::Error;
use vl_global::AudioDevices;
use vl_linux_backend::event_parameters;
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
impl Module for LinuxModule {
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

impl DeviceModule for LinuxModule {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>
    {
        Box::pin(async move {
            // call the method with no confirm
            if let Some(client) = &self._client {
                let result = client
                    .call(
                        BROKER_NAME,
                        "get_devices",
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
            Err(Box::new(LinuxModuleError::BackendServiceNotStarted)
                as Box<dyn std::error::Error>)
        })
    }

    fn is_capable_of_linking(&self) -> bool {
        true
    }

    fn link_device(
        &self,
        input_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        Box::pin(async move {
            if let Some(client) = &self._client {
                let result = client
                    .call(
                        BROKER_NAME,
                        "link_devices",
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
                    .map_err(|e| LinuxModuleError::FailedToLink(e))?;
                Ok(())
            } else {
                Err(Box::new(
                    LinuxModuleError::BackendServiceNotStarted,
                ) as Box<dyn std::error::Error>)
            }
        })
    }
    fn unlink_device(
        &self,
        input_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        Box::pin(async move {
            // call the method with no confirm
            if let Some(client) = &self._client {
                let result = client
                .call(
                    BROKER_NAME,
                    "unlink_devices",
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
                response.result.map_err(|e| {
                    LinuxModuleError::FailedToUnlink(e)
                })?;
                Ok(())
            } else {
                Err(Box::new(
                    LinuxModuleError::BackendServiceNotStarted,
                ) as Box<dyn std::error::Error>)
            }
        })
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

impl TtsModule for LinuxModule {
    fn speak(
        &self,
        text: String,
    ) -> futures::future::BoxFuture<
        Result<(), Box<dyn std::error::Error>>,
    > {
        Box::pin(async move {
            if let Some(client) = &self._client {
                let result = client
                    .call(
                        BROKER_NAME,
                        "speak",
                        rmp_serde::to_vec_named(
                            &event_parameters::RequestTTS {
                                phrase: text,
                                pitch: 48,
                                volume: 128,
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
                response.result.map_err(|e| {
                    LinuxModuleError::FailedToSpeak(e)
                })?;
                Ok(())
            } else {
                Err(Box::new(
                    LinuxModuleError::BackendServiceNotStarted,
                ) as Box<dyn std::error::Error>)
            }
        })
    }

    fn stop_speaking(
        &self,
    ) -> futures::future::BoxFuture<
        Result<(), Box<dyn std::error::Error>>,
    > {
        panic!("not implemented");
    }
}
