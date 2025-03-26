use std::sync::{Arc, Mutex};

use super::utils::{val, val_or, UNKNOWN_STR};
use libspa::utils::dict::DictRef;
use pipewire::registry::GlobalObject;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortError {
    #[error(
        "Port {0} could not be linked into Port {1}. Reason: {2}"
    )]
    LinkError(String, String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioChannel {
    MONO,
    FL,  // Front Left
    FR,  // Front Right
    FC,  // Front Center
    LFE, // Subwoofer
    SL,  // Side Left
    SR,  // Side Right
    RL,  // Rear Left
    RR,  // Rear Right
    TFL, // Top Front Left (Atmos)
    TFR, // Top Front Right (Atmos)
    Unknown,
}
impl AudioChannel {
    fn from_str(s: &str) -> Self {
        match s {
            "MONO" => AudioChannel::MONO,
            "FL" => AudioChannel::FL,
            "FR" => AudioChannel::FR,
            "FC" => AudioChannel::FC,
            "LFE" => AudioChannel::LFE,
            "SL" => AudioChannel::SL,
            "SR" => AudioChannel::SR,
            "RL" => AudioChannel::RL,
            "RR" => AudioChannel::RR,
            "TFL" => AudioChannel::TFL,
            "TFR" => AudioChannel::TFR,
            UNKNOWN_STR => AudioChannel::Unknown,
            _ => {
                log::warn!("An audio channel of type {s} has been found. That was totally not supposed to happen");
                AudioChannel::Unknown
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PortDirection {
    In,
    Out,
}
impl PortDirection {
    fn from_str(s: &str) -> Self {
        match s {
            "in" => PortDirection::In,
            "out" => PortDirection::Out,
            _ => panic!(
                "A port of direction {s} has been found. That was totally not supposed to happen"
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Port {
    pub id: u32,
    pub name: String,
    pub direction: PortDirection,
    pub alias: String,
    pub group: String,
    pub object_serial: u32,
    pub object_path: String,
    // pub format_dsp: String,
    /// The node this port belongs to
    pub node_id: u32,
    pub audio_channel: AudioChannel,
    // _core: Arc<Mutex<pipewire::core::Core>>,
    // // Optional fields (only present in some entries)
    // pub port_monitor: Option<String>,
    // pub port_physical: Option<String>,
    // pub port_terminal: Option<String>,
}
impl Port {
    pub fn new(port_dict: &GlobalObject<&DictRef>) -> Self {
        let props = port_dict.props.unwrap();
        let audio_channel =
            val_or(props, "audio.channel", UNKNOWN_STR);
        let port = Port {
            id: val(props, "port.id").parse().unwrap_or(u32::MAX),
            name: val(props, "port.name"),
            direction: PortDirection::from_str(&val(
                props,
                "port.direction",
            )),
            alias: val(props, "port.alias"),
            group: val(props, "port.group"),
            object_serial: val(props, "object.serial")
                .parse()
                .unwrap_or(u32::MAX),
            object_path: val(props, "object.path"),
            node_id: val(props, "node.id")
                .parse()
                .unwrap_or(u32::MAX),
            audio_channel: AudioChannel::from_str(&audio_channel),
        };
        log::debug!(
            "Creating new Port from global object: {:?}",
            port.name
        );
        port
    }

    /// Connect the current port into another, assuming that the other port is an input port.
    pub fn link_port(
        &self,
        input_port: &Self,
        core: Arc<Mutex<pipewire::core::Core>>,
    ) -> Result<(), PortError> {
        if self.direction != PortDirection::Out {
            return Err(PortError::LinkError(
                self.name.clone(),
                input_port.name.clone(),
                format!("{} is not an output port", self.name),
            ));
        }
        if input_port.direction != PortDirection::In {
            return Err(PortError::LinkError(
                self.name.clone(),
                input_port.name.clone(),
                format!("{} is not an input port", self.name),
            ));
        }
        let core = core.lock().expect("Failed to lock core");

        core.create_object::<pipewire::link::Link>(
            "link-factory",
            &pipewire::properties::properties! {
                "link.output.port" => self.node_id.to_string(),
                "link.input.port" => input_port.id.to_string(),
                "link.output.node" => self.node_id.to_string(),
                "link.input.node" => input_port.node_id.to_string(),
            },
        );

        log::debug!(
            "Port {} linked to port {}",
            self.name,
            input_port.name
        );

        Ok(())
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        log::debug!("Port {}({}) was removed", self.name, self.id);
    }
}
