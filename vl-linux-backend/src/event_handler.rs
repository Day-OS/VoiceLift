use crate::event_parameters;
use crate::event_parameters::ResponseDevices;
use crate::PIPEWIRE_MANAGER;
use busrt::rpc::RpcError;
use busrt::Frame;
use busrt::{
    async_trait,
    rpc::{RpcEvent, RpcHandlers, RpcResult},
};
use easy_pw::port::PortDirection;

fn evt_get_devices(event: RpcEvent) -> RpcResult {
    let _: event_parameters::RequestDevices =
        rmp_serde::from_slice(event.payload())?;

    let manager = PIPEWIRE_MANAGER
        .get()
        .expect("PipeWireManager not initialized")
        .lock()
        .expect("PipeWireManager Mutex is poisoned");

    let objects = manager.get_objects();

    // Create Response Struct
    let mut response_device = ResponseDevices {
        input_devices: vec![],
        output_devices: vec![],
    };

    let objects =
        objects.lock().expect("PipeWireObjects Mutex is poisoned");

    // Fill Response Struct
    for node in objects.nodes.iter() {
        for port in node.ports.iter() {
            if port.direction == PortDirection::In {
                if response_device.input_devices.contains(&node.name)
                {
                    continue;
                }
                response_device.input_devices.push(node.name.clone());
            } else if port.direction == PortDirection::Out {
                if response_device.output_devices.contains(&node.name)
                {
                    continue;
                }
                response_device
                    .output_devices
                    .push(node.name.clone());
            }
        }
    }

    let response = rmp_serde::to_vec(&response_device)?;

    Ok(Some(response))
}

pub(crate) struct EventHandler {}

#[async_trait]
impl RpcHandlers for EventHandler {
    // RPC call handler. Will react to the "test" (any params) and "ping" (will parse params as
    // msgpack and return the "message" field back) methods
    async fn handle_call(&self, event: RpcEvent) -> RpcResult {
        match event.parse_method()? {
            "get_devices" => evt_get_devices(event),
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
