use crate::events;
use busrt::rpc::RpcError;
use busrt::Frame;
use busrt::{
    async_trait,
    rpc::{RpcEvent, RpcHandlers, RpcResult},
};

pub(crate) struct EventHandler {}

#[async_trait]
impl RpcHandlers for EventHandler {
    // RPC call handler. Will react to the "test" (any params) and "ping" (will parse params as
    // msgpack and return the "message" field back) methods
    async fn handle_call(&self, event: RpcEvent) -> RpcResult {
        let parse_method = event.parse_method()?;
        let event_name = parse_method.to_owned();
        log::info!("Handling Event: {}", event_name);

        let result = match parse_method {
            "get_devices" => {
                events::get_devices::evt_get_devices(event)
            }
            "link_devices" => {
                events::link_devices::evt_link_devices(event)
            }
            "unlink_devices" => {
                events::unlink_devices::evt_unlink_devices(event)
            }
            "speak" => events::tts::evt_tts(event),
            _ => Err(RpcError::method(Some(
                "Event not implemented".as_bytes().to_vec(),
            ))),
        };
        log::info!("Event {} handled successfully.", event_name);
        result
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
