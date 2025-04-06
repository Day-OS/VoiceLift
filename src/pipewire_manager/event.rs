use std::{rc::Rc, sync::{Arc, Mutex}};

use pipewire::core::Core;

use super::objects::PipeWireObjects;

/// Events that is received by the main thread.
#[derive(Debug)]
pub enum ConnectorEvent {
    None,
}

/// Events that is received by the PipeWire Backend thread.
#[derive(Debug)]
pub enum PipeWireEvent {
    LinkCommand(u32, u32),
}

impl PipeWireEvent {
    pub fn handle(
        &self,
        objects: Arc<Mutex<PipeWireObjects>>,
        core: Rc<Mutex<Core>>,
    ) {
        log::debug!("Handling PipeWireEvent: {:#?}", self);
        match self {
            PipeWireEvent::LinkCommand(input_id, target_id) => {
                let objects = objects.lock();
                if let Err(e) = objects {
                    log::error!("Failed to lock objects: {}", e);
                    return;
                }
                let mut objects = objects.unwrap();

                // Implement the logic to handle the link command
                // This might involve finding the nodes and ports by their IDs and linking them
                // For example:
                let input_node =
                    objects.find_node_by_id(*input_id).cloned();
                let target_node = objects.find_node_by_id(*target_id);

                if input_node.is_none() || target_node.is_none() {
                    log::error!("One or both nodes not found for IDs: {} and {}", input_id, target_id);
                    return;
                }

                let mut input_node = input_node.unwrap();
                let target_node = target_node.unwrap();
                if let Err(e) = input_node.link_device(core, target_node.clone()){
                    log::error!("Failed to link devices: {}", e);
                }
            }
        }
    }
}
