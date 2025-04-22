use crate::event_parameters::ResponseDevices;
use crate::event_parameters::{self, ResponseDeviceLinkage};
use crate::{events, PIPEWIRE_MANAGER};
use busrt::rpc::RpcError;
use busrt::Frame;
use busrt::{
    async_trait,
    rpc::{RpcEvent, RpcHandlers, RpcResult},
};
use easy_pw::port::PortDirection;
use vl_global::AudioDevices;

pub(crate) struct EventHandler {}

#[async_trait]
impl RpcHandlers for EventHandler {
    // RPC call handler. Will react to the "test" (any params) and "ping" (will parse params as
    // msgpack and return the "message" field back) methods
    async fn handle_call(&self, event: RpcEvent) -> RpcResult {
        match event.parse_method()? {
            "get_devices" => {
                events::get_devices::evt_get_devices(event)
            }
            "link_devices" => {
                events::get_devices::evt_get_devices(event)
            }
            _ => Err(RpcError::method(Some(
                "Event not implemented".as_bytes().to_vec(),
            ))),
        }
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
