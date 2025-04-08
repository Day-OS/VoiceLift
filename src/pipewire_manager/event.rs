use std::{rc::Rc, sync::{Arc, Mutex}};

use pipewire::{core::Core, registry::Registry};

use super::{link::Link, objects::PipeWireObjects};

/// Events that is received by the main thread.
#[derive(Debug, PartialEq, Clone)]
pub enum ConnectorEvent {
    None,
    LinkUpdate(u32, u32),
}

/// Events that is received by the PipeWire Backend thread.
#[derive(Debug, PartialEq)]
pub enum PipeWireEvent {
    LinkCommand(u32, u32),
    UnlinkCommand(u32, u32),
}

impl ToString for PipeWireEvent {
    fn to_string(&self) -> String {
        match self {
            PipeWireEvent::LinkCommand(source_id, target_id) => format!("LinkCommand({}, {})", source_id, target_id),
            PipeWireEvent::UnlinkCommand(source_id, target_id) => format!("UnlinkCommand({}, {})", source_id, target_id),
        }
    }
}

impl PipeWireEvent {
    #[allow(unreachable_patterns)]
    pub fn handle(
        &self,
        _event_locker: Arc<Mutex<()>>,
        objects: Arc<Mutex<PipeWireObjects>>,
        core: Rc<Mutex<Core>>,
        registry: Rc<Mutex<Registry>>
    ) {
        let event_locker = _event_locker.lock().unwrap();
        log::debug!("(Pipewire) Handling Event: {:#?}", self);
        match self {
            PipeWireEvent::LinkCommand(source_id, target_id) => {
                let _ = &PipeWireEvent::_link_command(objects, core, *source_id, *target_id);
            }
            PipeWireEvent::UnlinkCommand(source_id, target_id) => {
                
                log::info!("Unlinking nodes {} and {}", source_id, target_id);
                let _ = &PipeWireEvent::_unlink_command(objects, registry, *source_id, *target_id);
            }
            _ => {
                log::warn!("Unhandled event: {:?}", self);
            }
        }
        drop(event_locker);
    }

    fn _link_command(
        objects: Arc<Mutex<PipeWireObjects>>,
        core: Rc<Mutex<Core>>, 
        source_id: u32, 
        target_id: u32){
        let objects = objects.lock();
        if let Err(e) = objects {
            log::error!("Failed to lock objects: {}", e);
            return;
        }
        let mut objects = objects.unwrap();
        
        let (input_node, target_node) = objects.find_two_nodes_by_id_mut(source_id, target_id);

        if input_node.is_none() || target_node.is_none() {
            log::error!("One or both nodes not found for IDs: {} and {}", source_id, target_id);
            return;
        }

        let input_node = input_node.unwrap();
        let target_node = target_node.unwrap();
        if let Err(e) = input_node.link_device(core, target_node){
            log::error!("Failed to link devices: {}", e);
        }
    }
    fn _unlink_command(
        objects: Arc<Mutex<PipeWireObjects>>,
        registry: Rc<Mutex<Registry>>,
        source_id: u32, 
        target_id: u32){
        let objects = objects.lock();
        if let Err(e) = objects {
            log::error!("Failed to lock objects: {}", e);
            return;
        }
        let mut objects = objects.unwrap();


        let link: Option<&mut Link> = objects.links
                                    .iter_mut()
                                    .find(|link|{
                                        link.output_node == source_id && 
                                        link.input_node == target_id
                                    });

        if let None = link {
            log::error!("Link not found for source ID: {} and target ID: {}", source_id, target_id);
            return;
        }

        let link_id = link.unwrap().id;
        log::info!("Found link with ID: {} while searching for source ID: {} and target ID: {}", link_id, source_id, target_id);

        objects.remove_link(link_id, Some(registry));
    }
}
