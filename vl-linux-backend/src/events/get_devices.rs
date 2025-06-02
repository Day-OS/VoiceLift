use crate::{
    event_parameters::{self},
    PIPEWIRE_MANAGER,
};
use busrt::rpc::{RpcEvent, RpcResult};
use easy_pw::port::PortDirection;
use vl_global::audio_devices::AudioDevices;

fn _evt_get_devices(event: RpcEvent) -> Result<AudioDevices, String> {
    // Verify if the event payload is of type RequestDevices
    let _: event_parameters::RequestDevices =
        rmp_serde::from_slice(event.payload()).map_err(|err| {
            format!("Failed to deserialize request: {}", err)
        })?;

    // Get PipeWire Manager Instance
    let manager = PIPEWIRE_MANAGER
        .get()
        .ok_or("PipeWireManager not initialized")?
        .read()
        .map_err(|e| {
            format!("Failed to lock PipeWireManager: {}", e)
        })?;

    let objects = manager.get_objects();

    // Create Response Struct
    let mut audio_device = AudioDevices {
        input_devices: vec![],
        output_devices: vec![],
    };

    let objects = objects.read().map_err(|e| {
        format!("Failed to lock PipeWireObjects: {}", e)
    })?;

    // Fill Response Struct
    for node in objects.nodes.iter() {
        for port in node.ports.iter() {
            if port.direction == PortDirection::In {
                if audio_device.input_devices.contains(&node.name) {
                    continue;
                }
                audio_device.input_devices.push(node.name.clone());
            } else if port.direction == PortDirection::Out {
                if audio_device.output_devices.contains(&node.name) {
                    continue;
                }
                audio_device.output_devices.push(node.name.clone());
            }
        }
    }
    drop(objects);
    drop(manager);
    Ok(audio_device)
}

pub fn evt_get_devices(event: RpcEvent) -> RpcResult {
    let result = _evt_get_devices(event);
    if let Err(e) = result.clone() {
        log::error!("Failed to get devices: {}", e);
    }
    let response =
        rmp_serde::to_vec(&event_parameters::ResponseDevices {
            result,
        })?;

    Ok(Some(response))
}
