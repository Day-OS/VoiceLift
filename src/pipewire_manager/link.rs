use std::{rc::Rc, sync::Mutex};

use super::utils::val;
use libspa::utils::dict::DictRef;
use pipewire::registry::{GlobalObject, Registry};

#[allow(dead_code)]
pub struct Link{
    pub(crate) id: u32,
    pub(crate) output_port: u32,
    pub(crate) input_port: u32,
    pub(crate) output_node: u32,
    pub(crate) input_node: u32,
}

impl Link {
    pub fn new(global: &GlobalObject<&DictRef>) -> Self {
        let props = global.props.unwrap();
        let node = Self {
            id: global.id,
            output_port: val(props, "link.output.port").parse().unwrap(),
            input_port:  val(props, "link.input.port").parse().unwrap(),
            output_node: val(props, "link.output.node").parse().unwrap(),
            input_node:  val(props, "link.input.node").parse().unwrap(),
        };
        log::debug!(
            "Creating new Link from global object: {:?}",
            node.id
        );
        node
    }
    pub async fn remove_link(&mut self, registry: Rc<Mutex<Registry>>){
        let registry = registry.lock();
        if let Err(e) = registry {
            log::error!("Failed to lock registry: {}", e);
            return;
        }
        let registry = registry.unwrap();
        let result = registry.destroy_global(self.id).into_async_result();
        if let Err(e) = result {
            log::error!("Failed to destroy global object: {}", e);
        } else {
            log::info!("Successfully destroyed global object: {}", self.id);
        }
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        log::debug!(
            "Link {} was removed",
            self.id,
        );
    }
}
