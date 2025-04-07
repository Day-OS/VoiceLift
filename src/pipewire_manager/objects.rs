use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use pipewire::registry::Registry;

use super::node::Node;
use super::port::Port;
use super::link::Link;

#[derive(Default)]
pub struct PipeWireObjects {
    pub(crate) nodes: Vec<Node>,
    pub(crate) links: Vec<Link>,
    pub(super) _ports_to_be_added: Vec<Port>,
}

impl PipeWireObjects {
    pub fn update_nodes(&mut self) {
        let mut nodes: HashMap<u32, (&mut Node, bool)> =
            HashMap::new();
        // Fill nodes
        if self.nodes.is_empty() || self._ports_to_be_added.is_empty()
        {
            return;
        }
        log::debug!("Nodes Quantity: {:?}", self.nodes.len());
        log::debug!(
            "Ports that need to be added: {:?}",
            self._ports_to_be_added.len()
        );
        for node in self.nodes.iter_mut() {
            nodes.insert(node.id, (node, false));
        }

        let mut ports_not_found: Vec<Port> = vec![];
        while let Some(port) = self._ports_to_be_added.pop() {

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
        self._ports_to_be_added.extend(ports_not_found);

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
    
    pub fn find_node_by_id(&self, id: u32) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == id || node.has_port_of_id(id))
    }
    
    #[allow(dead_code)]
    pub fn find_node_by_id_mut(&mut self, id: u32) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.id == id || node.has_port_of_id(id))
    }

    pub fn find_two_nodes_by_id_mut(&mut self, first_id: u32, second_id: u32)-> (Option<&mut Node>, Option<&mut Node>){
        let mut first:  Option<&mut Node> = None;
        let mut second: Option<&mut Node> = None;

        for node in &mut self.nodes {
            if node.id == first_id {
                first = Some(node);
            }
            else if node.id == second_id {
                second = Some(node)
            }
        }
        (first, second)
    }

    #[allow(dead_code)]
    pub fn find_links_by_id(&self, id: u32) -> Option<&Link> {
        self.links.iter().find(|link| link.id == id)
    }

    pub fn find_links_by_id_mut(&mut self, id: u32) -> Option<&mut Link> {
        self.links.iter_mut().find(|link| link.id == id)
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
    /// Removes a link from the list of links and optionally from the registry. 
    /// If registry is None, then it will not remove the link from the registry.
    pub fn remove_link(&mut self, id:u32, registry: Option<Rc<Mutex<Registry>>>){
        let link = self.links.iter_mut().find(|link| link.id != id);
        if link.is_none() {
            log::error!("Failed to find link with id {}", id);
            return;
        }
        if registry.is_some() { 
            let link = link.unwrap();
            link.remove_link(registry.unwrap()); 
        }
        let index = self.links.iter().position(|link| link.id == id).unwrap(); 
        self.links.remove(index);
    }
}
