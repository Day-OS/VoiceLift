use crate::{
    event_parameters::{self},
    PIPEWIRE_MANAGER,
};
use busrt::rpc::{RpcEvent, RpcResult};

fn _evt_unlink_devices(event: RpcEvent) -> Result<(), String> {
    // Verify if the event payload is of type RequestDevices
    let event: event_parameters::RequestDeviceUnLinkage =
        rmp_serde::from_slice(event.payload()).map_err(|err| {
            format!("Failed to deserialize request: {}", err)
        })?;

    // Get PipeWire Manager Instance
    let manager = PIPEWIRE_MANAGER
        .get()
        .ok_or("PipeWireManager not initialized")?
        .lock()
        .map_err(|e| {
            format!("Failed to lock PipeWireManager: {}", e)
        })?;

    let objects = manager.get_objects();

    let mut objects = objects.lock().map_err(|e| {
        format!("Failed to lock PipeWireObjects: {}", e)
    })?;

    // Find Objects
    let first_device = objects
        .find_node_by_name(&event.first_device)
        .ok_or("First device not found")?
        .id;
    let second_device = objects
        .find_node_by_name(&event.second_device)
        .ok_or("Second device not found")?
        .id;
    drop(objects);

    manager.unlink_nodes(first_device, second_device);
    drop(manager);

    Ok(())
}

pub fn evt_unlink_devices(event: RpcEvent) -> RpcResult {
    let result = _evt_unlink_devices(event);
    if let Err(e) = result.clone() {
        log::error!("Failed to unlink devices: {}", e);
    }
    let response = rmp_serde::to_vec(
        &event_parameters::ResponseDeviceUnLinkage { result },
    )?;

    Ok(Some(response))
}
