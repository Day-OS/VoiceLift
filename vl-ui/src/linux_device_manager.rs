use crate::{
    device_linker::DeviceLinker,
    device_manager::{AudioDevices, DeviceManager},
};
use busrt::ipc::{Client, Config};
use busrt::rpc::{Rpc, RpcClient};
use busrt::{empty_payload, QoS};
use futures::future::BoxFuture;
use serde::Deserialize;
use std::collections::BTreeMap;
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

            let audio_devices = AudioDevices {
                input_devices: devices.input_devices,
                output_devices: devices.output_devices,
            };
            Ok(audio_devices)
        })
    }
}

impl DeviceLinker for LinuxDeviceManager {
    fn link_device(
        &self,
        output_device: String,
        input_device: String,
    ) -> bool {
        todo!()
    }
}
