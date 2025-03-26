use std::sync::{Arc, Mutex};

use super::{
    port::Port,
    utils::{val, val_opt},
};
use libspa::utils::dict::DictRef;
use pipewire::permissions::PermissionFlags;
use pipewire::registry::GlobalObject;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub nick: Option<String>,
    pub permissions: PermissionFlags,
    pub version: u32,
    pub object_serial: String,
    pub factory_id: Option<String>,
    // Optional fields (wrapped in Option)
    pub object_path: Option<String>,
    pub client_id: Option<String>,
    pub device_id: Option<String>,
    pub priority_session: Option<String>,
    pub priority_driver: Option<String>,
    pub media_class: Option<String>,
    pub media_role: Option<String>,
    pub client_api: Option<String>,
    pub application_name: Option<String>,
    pub(crate) ports: Vec<Port>,
}

impl Node {
    pub fn new(global: &GlobalObject<&DictRef>) -> Self {
        let props = global.props.unwrap();
        let node = Node {
            id: global.id,
            name: val(props, "node.name"),
            description: val_opt(props, "node.description"),
            nick: val_opt(props, "node.nick"),
            permissions: global.permissions,
            version: global.version,
            object_serial: val(props, "object.serial"),
            factory_id: val_opt(props, "factory.id"),
            object_path: val_opt(props, "object.path"),
            client_id: val_opt(props, "client.id"),
            device_id: val_opt(props, "device.id"),
            priority_session: val_opt(props, "priority.session"),
            priority_driver: val_opt(props, "priority.driver"),
            media_class: val_opt(props, "media.class"),
            media_role: val_opt(props, "media.role"),
            client_api: val_opt(props, "client.api"),
            application_name: val_opt(props, "application.name"),
            ports: vec![],
        };
        log::debug!(
            "Creating new Node from global object: {:?}",
            node.name
        );
        node
    }

    pub fn get_port_names(&self) -> Vec<String> {
        self.ports.iter().map(|port| port.name.clone()).collect()
    }

    pub fn get_port_by_id(&mut self, port_id: u32) -> Option<&Port> {
        self.ports.iter().find(|port| port.id == port_id)
    }
    pub fn add_port(&mut self, port: Port) {
        self.ports.push(port.clone());
    }

    pub fn has_port(&self, port: &Port) -> bool {
        self.has_port_of_id(port.id)
    }

    pub fn has_port_of_id(&self, port_id: u32) -> bool {
        self.ports.iter().any(|p| p.id == port_id)
    }

    pub fn link_device(&mut self, input_device: Self) {

        // Implement the logic to link this node to another node
    }
}
impl Drop for Node {
    fn drop(&mut self) {
        log::debug!("Node {}({}) was removed", self.name, self.id);
    }
}
