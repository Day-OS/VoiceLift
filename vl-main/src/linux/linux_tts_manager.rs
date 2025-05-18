use busrt::ipc::{Client, Config};
use busrt::rpc::{Rpc, RpcClient};
use busrt::QoS;
use std::fmt::Debug;
use vl_linux_backend::event_parameters;
const BROKER_NAME: &str = ".broker";

use crate::base_managers::tts_manager::TtsManager;

pub struct LinuxTtsManager {
    _client: RpcClient,
}

impl Debug for LinuxTtsManager {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("LinuxTtsManager")
            .field("_client", &"RpcClient")
            .finish()
    }
}

impl LinuxTtsManager {
    async fn new_client(
    ) -> Result<RpcClient, Box<dyn std::error::Error>> {
        let name = "voice-lift-tts.client";
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

impl TtsManager for LinuxTtsManager {
    fn speak(
        &self,
        text: String,
    ) -> futures::future::BoxFuture<
        Result<(), Box<dyn std::error::Error>>,
    > {
        Box::pin(async move {
            // call the method with no confirm
            let result = self
                ._client
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
            response.result?;
            Ok(())
        })
    }
}
