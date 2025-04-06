use event::PipeWireEvent;
use libspa::utils::dict::DictRef;
use node::Node;
use objects::PipeWireObjects;
use pipewire as pw;
use pipewire::channel;
use pipewire::core::Core;
use pipewire::registry::GlobalObject;
use port::Port;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
mod event;
mod node;
pub mod objects;
mod port;
mod utils;

pub struct PipeWireManager {
    pub(crate) objects: Arc<Mutex<PipeWireObjects>>,
    pub _main_thread: thread::JoinHandle<()>,
    pub _receiver: mpsc::Receiver<event::ConnectorEvent>,
    _sender: channel::Sender<event::PipeWireEvent>,
}

impl Default for PipeWireManager {
    fn default() -> Self {
        let (main_sender, main_receiver) =
            mpsc::channel::<event::ConnectorEvent>();
        let (pw_sender, pw_receiver) =
            channel::channel::<event::PipeWireEvent>();
        // Store nodes in thread-safe container
        let nodes = Arc::new(Mutex::new(PipeWireObjects::default()));
        Self {
            objects: nodes.clone(),
            _main_thread: Self::_start_thread(
                main_sender,
                pw_receiver,
                nodes.clone(),
            ),
            _receiver: main_receiver,
            _sender: pw_sender,
        }
    }
}

impl PipeWireManager {
    fn _start_thread(
        _sender: mpsc::Sender<event::ConnectorEvent>,
        _receiver: channel::Receiver<event::PipeWireEvent>,
        nodes: Arc<Mutex<PipeWireObjects>>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            // Initialize PipeWire
            pw::init();
            let mainloop = pw::main_loop::MainLoop::new(None)
                .expect("Failed to create main loop");
            let context = pw::context::Context::new(&mainloop)
                .expect("Failed to create context");
            let core = context
                .connect(None)
                .expect("Failed to connect to core");
            let registry =
                core.get_registry().expect("Failed to get registry");

            // Clone for use in callback
            let nodes_clone = nodes.clone();
            let nodes_clone_remove = nodes.clone();
            let nodes_clone_event = nodes.clone();

            let core_mutex: Arc<Mutex<Core>> =
                Arc::new(Mutex::new(core));

            // Add registry listener
            let _listener = registry
                .add_listener_local()
                .global(move |global| {
                    Self::_pw_event_handler(
                        global,
                        &nodes_clone.clone(),
                    )
                })
                .global_remove(move |object_id| {
                    Self::pw_remove_event_handler(
                        object_id,
                        &nodes_clone_remove,
                    )
                })
                .register();

            let _receiver =
                _receiver.attach(mainloop.loop_(), move |event| {
                    log::debug!("Handling PipeWireEvent");

                    let objects = nodes_clone_event.clone();
                    let core = core_mutex.clone();
                    event.handle(objects, core)
                });

            let timer = mainloop.loop_().add_timer(move |_| {
                let _ = _sender.send(event::ConnectorEvent::None);
            });
            timer.update_timer(
                Some(Duration::from_millis(1)), // Send the first message immediately
                Some(Duration::from_millis(100)),
            );

            // Process events to populate nodes
            mainloop.run();
        })
    }
    fn _pw_event_handler(
        global: &GlobalObject<&DictRef>,
        objects: &Arc<Mutex<PipeWireObjects>>,
    ) {
        // Filter by only node ones
        let mut objects_guard = objects.lock().unwrap();
        match global.type_ {
            pw::types::ObjectType::Node => {
                let node = Node::new(global);
                objects_guard.nodes.push(node);
            }
            pw::types::ObjectType::Port => {
                let port = Port::new(global);
                objects_guard.ports_to_be_added.push(port);
                log::info!("Received PORT event: {:?} \n{:#?}", global, global.props)

            }
            _ => {
                log::info!("Received non-handled event: {:?} \n{:#?}", global.type_, global.props)
            }
        }
        objects_guard.update_nodes();
    }
    fn pw_remove_event_handler(
        object_id: u32,
        objects: &Arc<Mutex<PipeWireObjects>>,
    ) {
        let mut objs = objects.lock().unwrap();
        PipeWireManager::remove_object(&mut objs, object_id);
    }

    fn remove_object(objects: &mut PipeWireObjects, obj_id: u32) {
        let mut id: Option<u32> = None;
        if let Some(node) = objects.find_node_by_id(obj_id) {
            id = Some(node.id);
        }
        if let Some(id) = id {
            objects.remove_node(id);
        }
    }

    fn _raise_event(&self, event: PipeWireEvent) {
        let a = self._sender.send(event).unwrap();
    }

    pub fn link_nodes(
        &self,
        first_node_id: u32,
        second_node_id: u32,
    ) {
        self._raise_event(PipeWireEvent::LinkCommand(
            first_node_id,
            second_node_id,
        ))
    }
}
