pub(crate) mod device_linker;
pub(crate) mod device_manager;
pub(crate) mod tts_manager;

use std::sync::Arc;

use async_lock::RwLock;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::ResMut;
use bevy_egui::egui;
use bevy_tokio_tasks::TokioTasksRuntime;
use device_linker::DeviceLinker;
use device_manager::DeviceManager;
use egui_notify::Toasts;
use futures::executor;
use futures::future::BoxFuture;
use paste::paste;
use std::fmt::Debug;
use tts_manager::TtsManager;

#[cfg(target_os = "linux")]
use crate::desktop::linux::linux_module;

pub trait Module: Debug + Send + Sync {
    fn is_started(&self) -> bool;

    // Initialize a module
    fn start(
        &mut self,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;
}

#[derive(Resource)]
pub struct ModuleManager {
    pub(super) toast: Toasts,
    pending_error_messages: Vec<String>,
    device_linkers: Vec<Arc<RwLock<dyn DeviceLinker>>>,
    device_managers: Vec<Arc<RwLock<dyn DeviceManager>>>,
    tts_managers: Vec<Arc<RwLock<dyn TtsManager>>>,
    selected_device_linker: Option<Arc<RwLock<dyn DeviceLinker>>>,
    selected_device_manager: Option<Arc<RwLock<dyn DeviceManager>>>,
    selected_tts_manager: Option<Arc<RwLock<dyn TtsManager>>>,
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! are_modules_started {
    ( $( $field:ident ),* ) => {
        paste! {
            $(
                pub fn [<is_ $field _started>](&self) -> bool {
                    if let Some(module) = &self.[<selected_ $field>] {
                        let module = executor::block_on(module.read());
                        return module.is_started();
                    }
                    false
                }
            )*
            pub fn is_started(&self) -> bool {
                let checks: Vec<bool> = vec![
                    $(
                        self.[<is_ $field _started>](),
                    )*
                ];

                let all_true = checks.iter().all(|&b| b);
                all_true
            }
        }
    };
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            toast: Toasts::default(),
            pending_error_messages: vec![],
            device_linkers: vec![],
            device_managers: vec![],
            tts_managers: vec![],
            selected_device_linker: None,
            selected_device_manager: None,
            selected_tts_manager: None,
        }
    }
    pub async fn initialize(&mut self) -> &mut Self {
        #[cfg(target_os = "linux")]
        {
            let linux_module = Arc::new(RwLock::new(
                linux_module::LinuxModule::new().await,
            ));

            self.device_linkers.push(linux_module.clone());
            self.selected_device_linker = Some(linux_module.clone());

            self.device_managers.push(linux_module.clone());
            self.selected_device_manager = Some(linux_module.clone());

            self.tts_managers.push(linux_module.clone());
            self.selected_tts_manager = Some(linux_module.clone())
        }
        self
    }
    are_modules_started! {device_manager, device_linker, tts_manager}

    pub fn error(&mut self, text: String) {
        self.pending_error_messages.push(text);
    }
    pub fn _throw_error_message(&mut self, ctx: &mut egui::Context) {
        for error in &self.pending_error_messages {
            self.toast.error(error);
            log::error!("{error}");
        }
        self.pending_error_messages.clear();
        self.toast.show(ctx);
    }
}

/// This system should initialize the module as soon as it starts... I hope
pub fn initialize_module_manager(
    mut module_manager: ResMut<ModuleManager>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let runtime = runtime.runtime();
    let result = runtime.block_on(module_manager.initialize());
    let linker = result.selected_device_linker.clone().unwrap();
    let mut linker = runtime.block_on(linker.write());
    let linker_start_result = runtime.block_on(linker.start());
    if let Err(e) = linker_start_result {
        log::error!("{e}");
        let error = "Linker did not start".to_owned();
        module_manager.error(error);
    }
}
