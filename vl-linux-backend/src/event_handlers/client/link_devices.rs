use crate::{piper::PiperTTSManager, PIPEWIRE_MANAGER};
use busrt::rpc::{RpcEvent, RpcResult};

use events::client::{RequestDeviceLinkage, ResponseDeviceLinkage};
use vl_linux_backend::events;

fn _evt_link_devices(event: RpcEvent) -> Result<(), String> {
    // Verify if the event payload is of type RequestDevices
    let event: RequestDeviceLinkage =
        rmp_serde::from_slice(event.payload()).map_err(|err| {
            format!("Failed to deserialize request: {err}")
        })?;
    // Get PipeWire Manager Instance
    let manager = PIPEWIRE_MANAGER
        .get()
        .ok_or("PipeWireManager not initialized")?
        .read()
        .map_err(|e| {
            format!("Failed to lock PipeWireManager: {e}")
        })?;

    let objects = manager.get_objects();

    let objects = objects.read().map_err(|e| {
        format!("Failed to lock PipeWireObjects: {e}")
    })?;

    // Find Objects
    let first_name = PiperTTSManager::get_handle_name();
    let second_name = event.target_device;
    let err_description = format!(
        "while trying to link {first_name} <==> {second_name}"
    );
    let first_device = objects
        .find_node_id_by_name(&first_name)
        .ok_or("First device not found")?;
    let second_device =
        objects.find_node_id_by_name(&second_name).ok_or(format!(
            "Second device not found {err_description}"
        ))?;
    drop(objects);

    manager.link_nodes(first_device, second_device);
    drop(manager);

    Ok(())
}

pub fn evt_link_devices(event: RpcEvent) -> RpcResult {
    let result = _evt_link_devices(event);
    if let Err(e) = result.clone() {
        log::error!("Failed to link devices: {e}");
    }
    let response =
        rmp_serde::to_vec(&ResponseDeviceLinkage { result })?;

    Ok(Some(response))
}
