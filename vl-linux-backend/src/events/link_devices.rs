use crate::{
    event_parameters::{self},
    piper::PiperTTSManager,
    PIPEWIRE_MANAGER,
};
use busrt::rpc::{RpcEvent, RpcResult};

fn _evt_link_devices(event: RpcEvent) -> Result<(), String> {
    // Verify if the event payload is of type RequestDevices
    let event: event_parameters::RequestDeviceLinkage =
        rmp_serde::from_slice(event.payload()).map_err(|err| {
            format!("Failed to deserialize request: {}", err)
        })?;

    println!("travou no pipwiremanager?");
    // Get PipeWire Manager Instance
    let manager = PIPEWIRE_MANAGER
        .get()
        .ok_or("PipeWireManager not initialized")?
        .read()
        .map_err(|e| {
            format!("Failed to lock PipeWireManager: {}", e)
        })?;
    println!("não ! :)");

    let objects = manager.get_objects();

    let objects = objects.read().map_err(|e| {
        format!("Failed to lock PipeWireObjects: {}", e)
    })?;

    // Find Objects
    let first_device = objects
        .find_node_id_by_name(&PiperTTSManager::get_handle_name())
        .ok_or("First device not found")?;
    let second_device = objects
        .find_node_id_by_name(&event.target_device)
        .ok_or("Second device not found")?;
    drop(objects);

    manager.link_nodes(first_device, second_device);
    drop(manager);

    Ok(())
}

pub fn evt_link_devices(event: RpcEvent) -> RpcResult {
    let result = _evt_link_devices(event);
    if let Err(e) = result.clone() {
        log::error!("Failed to link devices: {}", e);
    }
    let response = rmp_serde::to_vec(
        &event_parameters::ResponseDeviceLinkage { result },
    )?;

    Ok(Some(response))
}
