use bevy::ecs::event::EventWriter;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::ResMut;
use busrt::Frame;
use busrt::rpc::RpcError;
use busrt::{
    async_trait,
    rpc::{RpcEvent, RpcHandlers, RpcResult},
};
use vl_linux_backend::events;
use vl_linux_backend::events::server::{self, ResponseServerEvent};

use crate::modules::module_event::ModuleEvent;

#[derive(Resource)]
pub(crate) struct LinuxModuleEventHandler {
    pending_events: Vec<ModuleEvent>,
}

pub fn linux_module_event_handler_update(
    mut handler: ResMut<LinuxModuleEventHandler>,
    mut event_w: EventWriter<ModuleEvent>,
) {
    if handler.pending_events.is_empty() {
        return;
    }
    while let Some(event) = handler.pending_events.pop() {
        event_w.write(event);
    }
}

impl LinuxModuleEventHandler {
    pub fn new() -> Self {
        Self {
            pending_events: vec![],
        }
    }
}

#[async_trait]
impl RpcHandlers for LinuxModuleEventHandler {
    // RPC call handler. Will react to the "test" (any params) and "ping" (will parse params as
    // msgpack and return the "message" field back) methods
    async fn handle_call(&self, event: RpcEvent) -> RpcResult {
        let parse_method = event.parse_method()?;
        let event_name = parse_method.to_owned();
        log::debug!("Handling Event: {}", event_name);

        let mut method_not_found = false;
        match parse_method {
            server::METHOD_DEVICE_LIST_UPDATED => {}
            _ => {
                method_not_found = true;
            }
        };

        // Return a generic resonse back
        if method_not_found {
            return Err(RpcError::method(Some(
                "Event not implemented".as_bytes().to_vec(),
            )));
        }

        log::info!("Event {} handled successfully.", event_name);

        Ok(Some(rmp_serde::to_vec(&ResponseServerEvent {})?))
    }
    // Handle RPC notifications
    async fn handle_notification(&self, event: RpcEvent) {
        println!(
            "Got RPC notification from {}: {}",
            event.sender(),
            std::str::from_utf8(event.payload())
                .unwrap_or("something unreadable")
        );
    }
    // handle broadcast notifications and topic publications
    async fn handle_frame(&self, frame: Frame) {
        println!(
            "Got non-RPC frame from {}: {:?} {:?} {}",
            frame.sender(),
            frame.kind(),
            frame.topic(),
            std::str::from_utf8(frame.payload())
                .unwrap_or("something unreadable")
        );
    }
}
