use crate::event_parameters::{self};
use busrt::rpc::{RpcEvent, RpcResult};

// fn _evt_tts(event: RpcEvent) -> Result<(), String> {
//     // Verify if the event payload is of type RequestDevices
//     let event: event_parameters::RequestTTS =
//         rmp_serde::from_slice(event.payload()).map_err(|err| {
//             format!("Failed to deserialize request: {}", err)
//         })?;

//     let objects = manager.get_objects();

//     let mut objects = objects.lock().map_err(|e| {
//         format!("Failed to lock PipeWireObjects: {}", e)
//     })?;

//     // Find Objects
//     let first_device = objects
//         .find_node_by_name(&event.first_device)
//         .ok_or("First device not found")?
//         .id;
//     let second_device = objects
//         .find_node_by_name(&event.second_device)
//         .ok_or("Second device not found")?
//         .id;
//     drop(objects);

//     manager.link_nodes(first_device, second_device);
//     drop(manager);

//     Ok(())
// }

// pub fn evt_tts(event: RpcEvent) -> RpcResult {
//     let result = _evt_link_devices(event);
//     if let Err(e) = result.clone() {
//         log::error!("Failed to link devices: {}", e);
//     }
//     let response =
//         rmp_serde::to_vec(&event_parameters::ResponseTTS { result })?;

//     Ok(Some(response))
// }
