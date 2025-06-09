use busrt::async_trait;
use busrt::ipc::{Client, Config};
use busrt::rpc::RpcClient;
use std::fmt::Debug;

use crate::modules::base::i_module::IModule;
use crate::modules::linux::error::LinuxModuleError;
use crate::modules::linux::event_handlers;

/// Resposible for linking devices in Linux with the help of a backend (vl-linux-backend)
pub struct LinuxModule {
    pub(super) _client: Option<RpcClient>,
}

impl LinuxModule {
    async fn new_client() -> anyhow::Result<RpcClient> {
        let name = "voice-lift-device.client";
        // create a new client instance
        let config = Config::new("/tmp/voicelift.sock", name);
        let client = Client::connect(&config).await.map_err(|e| {
            LinuxModuleError::FailedToConnectIntoSocket(e.to_string())
        })?;
        // create RPC with no handlers
        let handlers =
            event_handlers::handler::LinuxModuleEventHandler::new();

        Ok(RpcClient::new(client, handlers))
    }
    pub async fn new() -> Self {
        Self { _client: None }
    }
}

#[async_trait]
impl IModule for LinuxModule {
    fn is_started(&self) -> bool {
        self._client.is_some()
    }

    async fn start(&mut self) -> anyhow::Result<()> {
        self._client = Some(Self::new_client().await?);
        Ok(())
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
