use std::collections::HashMap;

use super::node::Node;
use super::port::Port;

#[derive(Default)]
pub struct PipeWireObjects {
    pub(crate) nodes: Vec<Node>,
    pub(super) ports_to_be_added: Vec<Port>,
}

impl PipeWireObjects {
    pub fn update_nodes(&mut self) {
        let mut nodes: HashMap<u32, (&mut Node, bool)> =
            HashMap::new();
        // Fill nodes
        if self.nodes.is_empty() || self.ports_to_be_added.is_empty()
        {
            return;
        }
        log::debug!("Nodes Quantity: {:?}", self.nodes.len());
        log::debug!(
            "Ports that need to be added: {:?}",
            self.ports_to_be_added.len()
        );
        for node in self.nodes.iter_mut() {
            nodes.insert(node.id, (node, false));
        }

        let mut ports_not_found: Vec<Port> = vec![];
        while let Some(port) = self.ports_to_be_added.pop() {

            let port_id = port.id;
            let node_id = port.node_id;

            if let Some(node) = nodes.get_mut(&node_id) {
                if node.0.has_port(&port) {
                    continue;
                }
                log::debug!(
                    "Adding port {} to node {}",
                    port_id,
                    node_id
                );
                node.0.add_port(port);
                node.1 = true;
            } else {
                log::error!("Port {} has no node", port_id);
                ports_not_found.push(port);
            }
        }

        // If the port was not found, then we reintegrate it into our ports_to_be_added list
        // That makes sure that it will not be deleted at this time
        self.ports_to_be_added.extend(ports_not_found);

        for (_, (node, updated)) in nodes.iter() {
            if !updated {
                continue;
            }
            log::debug!(
                "Node {}({}) was updated | Ports: {:#?}",
                node.name,
                node.id,
                node.get_port_names()
            );
        }

        // DEBUG ALL NODES:
        // let str_nodes: Vec<String> = self.nodes.iter().map(|node| format!("Node {}({}) | Ports: {:#?}",
        // node.name,
        // node.id,
        // node.get_port_names())).collect();
        // log::debug!("{:#?}", str_nodes);
    }
    // This will search any id that is somehow linked with a node
    // That means that it will scan all nodes and all their ports
    // and return the node if it finds it
    pub fn find_node_by_id(&mut self, id: u32) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.id == id || node.has_port_of_id(id))
    }

    pub fn find_node_by_name(
        &mut self,
        name: &str,
    ) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.name == name)
    }

    pub fn remove_node(&mut self, id: u32) {
        if let Some(index) =
            self.nodes.iter().position(|n| n.id == id)
        {
            self.nodes.remove(index);
        }
    }

    pub fn print_nodes(&self) {
        self.nodes.iter().for_each(|node| {
            log::info!("=======================\nNode ID: {}, \nNode Name: {} \nNode Description {:?} \nPorts: {:?}", node.id, node.name, node.description, node.get_port_names());
        });
    }
}
