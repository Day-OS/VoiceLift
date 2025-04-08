use event::{ConnectorEvent, PipeWireEvent};
use libspa::utils::dict::DictRef;
use link::Link;
use node::Node;
use objects::PipeWireObjects;
use pipewire as pw;
use pipewire::channel;
use pipewire::core::Core;
use pipewire::registry::{GlobalObject, Registry};
use port::Port;
use std::rc::Rc;
use std::sync::mpsc::TryRecvError;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
mod event;
mod node;
pub mod objects;
mod port;
mod utils;
mod link;

pub struct PipeWireManager {
    pub(crate) objects: Arc<Mutex<PipeWireObjects>>,
    pub _main_thread: thread::JoinHandle<()>,
    pub _receiver: mpsc::Receiver<event::ConnectorEvent>,
    _sender: channel::Sender<event::PipeWireEvent>,
    pub _event_locker: Arc<Mutex<()>>,
}

impl Default for PipeWireManager {
    fn default() -> Self {
        let (main_sender, main_receiver) =
            mpsc::channel::<event::ConnectorEvent>();
        let (pw_sender, pw_receiver) =
            channel::channel::<event::PipeWireEvent>();
        // Store nodes in thread-safe container
        let nodes = Arc::new(Mutex::new(PipeWireObjects::default()));
        let event_locker = Arc::new(Mutex::new(()));
        
        Self {
            objects: nodes.clone(),
            _main_thread: Self::_start_thread(
                event_locker.clone(),
                main_sender,
                pw_receiver,
                nodes.clone(),
            ),
            _receiver: main_receiver,
            _sender: pw_sender,
            _event_locker: event_locker,
        }
    }
}

impl PipeWireManager {
    fn _start_thread(
        _event_locker: Arc<Mutex<()>>,
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

            let core_mutex: Rc<Mutex<Core>> =
            Rc::new(Mutex::new(core));

            let registry_mutex: Rc<Mutex<Registry>> = Rc::new(Mutex::new(registry));
            
            let registry_lock = registry_mutex.lock().unwrap();
            
            // Add registry listener
            let _listener = registry_lock
                .add_listener_local()
                .global(move |global| {
                    Self::_pw_event_handler(
                        global,
                        &nodes_clone.clone(),
                        &_sender,
                    )
                })
                .global_remove(move |object_id| {
                    Self::_pw_remove_event_handler(
                        object_id,
                        &nodes_clone_remove,
                    )
                })
                .register();

            drop(registry_lock);

            let _receiver =
                _receiver.attach(mainloop.loop_(), move |event| {
                    let objects = nodes_clone_event.clone();
                    let core = core_mutex.clone();
                    event.handle(
                        _event_locker.clone(),objects, core, registry_mutex.clone())
                });

            // Process events to populate nodes
            mainloop.run();
        })
    }
    fn _pw_event_handler(
        global: &GlobalObject<&DictRef>,
        objects: &Arc<Mutex<PipeWireObjects>>,
        _sender: &mpsc::Sender<ConnectorEvent>,
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
                objects_guard._ports_to_be_added.push(port);
                log::debug!("(Pipewire)Received PORT event: {:?} \n{:#?}", global, global.props);
            }
            pw::types::ObjectType::Link => {
                let link = Link::new(global);
                log::debug!("(Pipewire) Received LINK event: {:?} \n{:#?}", global, global.props);
                let first_id = link.output_node;
                let second_id = link.input_node;
                objects_guard.links.push(link);
                let _result = 
                    _sender.send(ConnectorEvent::LinkUpdate(first_id, second_id));
            }
            _ => {
                log::debug!("(Pipewire)Received non-handled event: {:?} \n{:#?}", global.type_, global.props);
                let _result = _sender.send(ConnectorEvent::None);
            }
        }
        objects_guard.update_nodes();
    }

    fn _pw_remove_event_handler(
        object_id: u32,
        objects: &Arc<Mutex<PipeWireObjects>>,
    ) {
        let mut objs = objects.lock().unwrap();
        PipeWireManager::remove_object(&mut objs, object_id);
    }

    fn _raise_event(&self, event: PipeWireEvent) {
        let event_info = event.to_string();
        if let Err(e) =  self._sender.send(event){
            log::error!("Failed to send event: {:?}", e);
        }
        log::debug!("Event raised: {:?}", event_info);
        let _thread_locker = self._event_locker.lock().unwrap();
    }

    fn remove_object(objects: &mut PipeWireObjects, obj_id: u32) {
        let mut link_id: Option<u32> = None;
        let mut node_id: Option<u32> = None;

        if let Some(link) = objects.find_links_by_id_mut(obj_id) {
            link_id = Some(link.id);
        }
        if let Some(node) = objects.find_node_by_id(obj_id) {
            node_id = Some(node.id);
        }

        if let Some(id) = link_id {
            objects.remove_link(id, None);
        }
        if let Some(id) = node_id {
            objects.remove_node(id);
        }
    }

    /// Create a link between two nodes
    /// The first one should have an output port and the second one an input port
    #[allow(dead_code)]
    pub fn link_nodes(
        &self,
        first_node_id: u32,
        second_node_id: u32,
    ) {
        self._raise_event(PipeWireEvent::LinkCommand(
            first_node_id,
            second_node_id,
        ));
        self.wait_for_event(|event: &ConnectorEvent|{
            *event == ConnectorEvent::LinkUpdate(first_node_id, second_node_id)
        });
    }


    /// Get the first link between two nodes and remove it
    #[allow(dead_code)]
    pub fn unlink_nodes(
        &self,
        first_node_id: u32,
        second_node_id: u32,
    ) {
        self._raise_event(PipeWireEvent::UnlinkCommand(
            first_node_id,
            second_node_id,
        ));
        self.wait_for_event(|event: &ConnectorEvent|{
            *event == ConnectorEvent::LinkUpdate(first_node_id, second_node_id)
        });
    }

    fn wait_for_event<F: Fn(&ConnectorEvent) -> bool>(&self, checker: F) {
        let mut event_result: ConnectorEvent = ConnectorEvent::None;
        // Lock the thread and wait for the event to be processed
        while checker(&event_result) == false {
            let result = self._receiver.try_recv();
            
            if let Err(e) = result{
                if e == TryRecvError::Disconnected {
                    log::error!("Failed to receive event: {}", e);
                }
                continue;
            }
            event_result = result.unwrap();
        };
        log::debug!("(Connector) Received event: {:?}", event_result)
    }
}
