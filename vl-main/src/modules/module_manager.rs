use super::base::device_module::DeviceModule;
use super::base::tts_module::TtsModule;
#[cfg(target_os = "linux")]
use super::linux::linux_module;
use crate::manager::Manager;
use crate::modules::base::module::Module;
use async_lock::RwLock;
use bevy::ecs::resource::Resource;
use bevy::platform::collections::HashMap;
use bevy::time::Timer;
use bevy::time::TimerMode;
use bevy_egui::egui;
use egui_file_dialog::FileDialog;
use egui_notify::Toasts;
use futures::executor;
use std::sync::Arc;
use std::time::Duration;
use vl_global::audio_devices::AudioDevices;
use vl_global::vl_config::ConfigError;
use vl_global::vl_config::ConfigManager;
use vl_global::vl_config::VlConfig;

#[derive(Resource)]
pub struct ModuleManager {
    pub config: Arc<RwLock<ConfigManager>>,
    pub file_dialog: Arc<RwLock<FileDialog>>,
    pub(super) toast: Toasts,
    pending_error_messages: Vec<String>,
    pub(crate) modules: HashMap<String, Module>,
    pub(crate) selected_device_module:
        Option<Arc<RwLock<dyn DeviceModule>>>,
    pub(crate) selected_tts_module:
        Option<Arc<RwLock<dyn TtsModule>>>,
    pub(crate) _timer: Timer,
    pub available_devices: AudioDevices,
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
    }
}
impl ModuleManager {
    pub fn new() -> Self {
        let app_config = Arc::new(RwLock::new(
            ConfigManager::new()
                .expect("Config should be initialized"),
        ));

        Self {
            file_dialog: Arc::new(RwLock::new(FileDialog::new())),
            config: app_config,
            toast: Toasts::default(),
            pending_error_messages: vec![],
            modules: HashMap::new(),
            selected_device_module: None,
            selected_tts_module: None,
            _timer: Timer::new(
                Duration::from_secs(1),
                TimerMode::Repeating,
            ),
            available_devices: AudioDevices::default(),
        }
    }

    pub async fn initialize(&mut self) -> &mut Self {
        #[cfg(target_os = "linux")]
        {
            let module = linux_module::LinuxModule::new().await;
            let linux_module = Arc::new(RwLock::new(module));

            let tts = Module::TtsModule(linux_module.clone());
            let device = Module::DeviceModule(linux_module.clone());
            self.modules
                .insert(tts.get_module_type().to_owned(), tts);
            self.modules
                .insert(device.get_module_type().to_owned(), device);
            self.selected_device_module = Some(linux_module.clone());
            self.selected_tts_module = Some(linux_module.clone())
        }

        // update config!

        self.update_config().unwrap();
        self
    }
    // #region Config
    pub fn update_config(&mut self) -> Result<(), ConfigError> {
        let config_clone = self.config.clone();
        let mut config = executor::block_on(config_clone.write());
        config.modify_and_save(|config| {
            self._update_config(config);
            Ok(())
        })?;
        Ok(())
    }

    fn _update_config(&mut self, config: &mut VlConfig) {
        if let Some(module) = &self.selected_device_module {
            let module = executor::block_on(module.read());
            let type_name = module.get_module_type();
            let name = module.get_screen_name();
            config
                .selected_modules
                .insert(type_name.to_owned(), name.to_owned());
        }
        if let Some(module) = &self.selected_tts_module {
            let module = executor::block_on(module.read());
            let type_name = module.get_module_type();
            let name = module.get_screen_name();
            config
                .selected_modules
                .insert(type_name.to_owned(), name.to_owned());
        }
    }

    pub fn reload_config(&mut self) {
        let app_config = Arc::new(RwLock::new(
            ConfigManager::new()
                .expect("Config should be initialized"),
        ));
        self.config = app_config;
    }
    // #endregion

    // #region Error

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

    // #endregion

    pub fn select_module(&mut self, module: Module) {
        match module {
            Module::TtsModule(rw_lock) => {
                self.selected_tts_module = Some(rw_lock.clone());
            }
            Module::DeviceModule(rw_lock) => {
                self.selected_device_module = Some(rw_lock.clone());
            }
        };
        self.update_config().unwrap();
    }

    pub fn is_started(&self) -> bool {
        let mut checks = vec![];
        if let Some(module) = &self.selected_device_module {
            let device = executor::block_on(module.read());
            checks.push(device.is_started());
        }
        if let Some(module) = &self.selected_tts_module {
            let device = executor::block_on(module.read());
            checks.push(device.is_started());
        }
        if checks.is_empty() {
            return false;
        }

        checks.iter().all(|&b| b)
    }

    async fn _get_linker_module(
        &mut self,
    ) -> Result<Arc<RwLock<dyn DeviceModule>>, &str> {
        let module = &mut self.selected_device_module;

        if module.is_none() {
            return Err(
                "relink_all_devices called while there's no active device module",
            );
        }
        let module_arc = module.clone().unwrap();

        let result = module_arc.clone();
        let module = module_arc.read().await;

        if !module.is_capable_of_linking() {
            return Err(
                "relink_all_devices called while this device module is not capable of linking devices",
            );
        }
        Ok(result)
    }

    pub async fn relink_all_devices(&mut self) {
        let module = self._get_linker_module().await;
        if let Err(e) = module {
            log::error!("{e}");
            return;
        }

        let module_arc = module.unwrap();
        let module = module_arc.write().await;

        let config_manager = self.config.read().await;
        let config_result = config_manager.read();
        if let Err(e) = config_result {
            log::error!("{e}");
            return;
        }

        let config = config_result.unwrap();
        for device in config.devices.input_devices {
            if let Err(e) = module.link_device(device).await {
                log::error!("{e}");
            }
        }
    }

    pub async fn unlink_device(&mut self, device: String) {
        let module = self._get_linker_module().await;
        if let Err(e) = module {
            log::error!("{e}");
            return;
        }
        let module_arc = module.unwrap();
        let module = module_arc.write().await;

        if let Err(e) = module.unlink_device(device).await {
            log::error!("{e}");
        }
    }
}

impl Manager for ModuleManager {
    fn modify_app(&mut self, app: &mut bevy::app::App) {
        #[cfg(target_os = "linux")]
        {
            use bevy::app::Update;

            use crate::modules::linux::event_handlers::handler::{
                LinuxModuleEventHandler,
                linux_module_event_handler_update,
            };
            app.insert_resource(LinuxModuleEventHandler::new());
            app.add_systems(
                Update,
                linux_module_event_handler_update,
            );
        }
    }
}
