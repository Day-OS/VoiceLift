use crate::base_managers::{
    device_linker::DeviceLinker, device_manager::DeviceManager,
};
use busrt::ipc::{Client, Config};
use busrt::rpc::{Rpc, RpcClient};
use busrt::QoS;
use futures::future::BoxFuture;
use std::fmt::Debug;
use vl_global::AudioDevices;
use vl_linux_backend::event_parameters;
const BROKER_NAME: &str = ".broker";

/// Resposible for linking devices in Linux with the help of a backend (vl-linux-backend)
pub struct LinuxDeviceManager {
    _client: RpcClient,
}

impl LinuxDeviceManager {
    async fn new_client(
    ) -> Result<RpcClient, Box<dyn std::error::Error>> {
        let name = "voice-lift.client";
        // create a new client instance
        let config = Config::new("/tmp/voicelift.sock", name);
        let client = Client::connect(&config).await?;
        // create RPC with no handlers
        Ok(RpcClient::new0(client))
    }
    pub async fn new() -> Self {
        let rpc = Self::new_client().await.unwrap();
        Self { _client: rpc }
    }
}

impl DeviceManager for LinuxDeviceManager {
    fn get_devices(
        &self,
    ) -> BoxFuture<Result<AudioDevices, Box<dyn std::error::Error>>>
    {
        Box::pin(async move {
            // call the method with no confirm
            let result = self
                ._client
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

            Ok(devices.result?)
        })
    }
}

impl DeviceLinker for LinuxDeviceManager {
    fn link_device(
        &self,
        output_device: String,
        input_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        Box::pin(async move {
            // call the method with no confirm
            let result = self
                ._client
                .call(
                    BROKER_NAME,
                    "link_devices",
                    rmp_serde::to_vec_named(
                        &event_parameters::RequestDeviceLinkage {
                            first_device: output_device,
                            second_device: input_device,
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
            response.result?;
            Ok(())
        })
    }
    fn unlink_device(
        &self,
        output_device: String,
        input_device: String,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        Box::pin(async move {
            // call the method with no confirm
            let result = self
                ._client
                .call(
                    BROKER_NAME,
                    "unlink_devices",
                    rmp_serde::to_vec_named(
                        &event_parameters::RequestDeviceUnLinkage {
                            first_device: output_device,
                            second_device: input_device,
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
            response.result?;
            Ok(())
        })
    }
}

impl Debug for LinuxDeviceManager {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("LinuxDeviceManager").finish()
    }
}
