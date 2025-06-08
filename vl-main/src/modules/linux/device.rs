use busrt::rpc::Rpc;
use busrt::{QoS, async_trait};
use vl_global::audio_devices::AudioDevices;
use vl_linux_backend::events::client::{
    self, METHOD_GET_DEVICES, METHOD_LINK_DEVICES,
    METHOD_UNLINK_DEVICES,
};

use crate::modules::base::device_module::DeviceModule;
use crate::modules::linux::BROKER_NAME;
use crate::modules::linux::error::LinuxModuleError;
use crate::modules::linux::linux_module::LinuxModule;

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
                        &client::RequestDevices {},
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

            let devices: client::ResponseDevices =
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
                        &client::RequestDeviceLinkage {
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

            let response: client::ResponseDeviceLinkage =
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
                        &client::RequestDeviceUnLinkage {
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

            let response: client::ResponseDeviceUnLinkage =
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
