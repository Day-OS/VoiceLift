use std::sync::{Arc, Mutex};

use crate::pipewire_manager::port::PortDirection;

use super::{
    port::{Port, PortError},
    utils::{val, val_opt},
};
use libspa::utils::dict::DictRef;
use pipewire::permissions::PermissionFlags;
use pipewire::registry::GlobalObject;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("")]
    PortError(#[from] PortError),
    #[error("Node {0} does not have a port with direction {1:?}")]
    IncorrectTypeOfChannelDirection(String, PortDirection),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

    #[allow(dead_code)]
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

    pub fn link_device(
        &mut self,
        core: Arc<Mutex<pipewire::core::Core>>,
        input_device: Self,
    ) -> Result<(), NodeError> {
        log::debug!(
            "Linking device \"{}\" to \"{}\"",
            self.name,
            input_device.name
        );

        // First we verify if self contains output ports
        if !self
            .ports
            .iter()
            .any(|port| port.direction == PortDirection::Out)
        {
            log::error!(
                "Node \"{}\" does not have any output ports",
                self.name
            );
            return Err(NodeError::IncorrectTypeOfChannelDirection(
                self.name.clone(),
                PortDirection::Out,
            ));
        }

        // Then we verify if input_device contains input ports
        if !input_device
            .ports
            .iter()
            .any(|port| port.direction == PortDirection::In)
        {
            log::error!("Node \"{}\" does not have any input ports | Available Ports: {:#?}", input_device.name, input_device.ports);
            return Err(NodeError::IncorrectTypeOfChannelDirection(
                input_device.name.clone(),
                PortDirection::In,
            ));
        }

        let mut were_matching_ports_found = false;

        // First we check if the two nodes have the same ammount
        // of channels and the same audio channels
        if self.ports.len() == input_device.ports.len() {
            for port in self.ports.iter() {
                if port.direction == PortDirection::In {
                    continue;
                }
                let matching_port = input_device
                    .ports
                    .iter()
                    .find(|p| p.audio_channel == port.audio_channel);
                if matching_port.is_none() {
                    continue;
                }
                port.link_port(core.clone(), matching_port.unwrap())?;
                were_matching_ports_found = true;
            }
        }
        if were_matching_ports_found {
            return Ok(());
        }

        // If no matching ports were found, we try to link the first output port in the node to the first input port in the input device
        let first_port = self
            .ports
            .iter()
            .find(|p| p.direction == PortDirection::Out);
        if let None = first_port {
            log::warn!("No output port found in node {}", self.name);
            return Err(NodeError::IncorrectTypeOfChannelDirection(
                input_device.name.clone(),
                PortDirection::In,
            ));
        }
        let first_port = first_port.unwrap();
        for other_port in input_device.ports.iter() {
            if other_port.direction != PortDirection::In {
                continue;
            }
            first_port.link_port(core.clone(), &other_port)?;
        }
        Ok(())
    }
}
impl Drop for Node {
    fn drop(&mut self) {
        log::debug!("Node {}({}) was removed", self.name, self.id);
    }
}
