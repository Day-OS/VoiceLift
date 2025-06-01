use super::device_module::DeviceModule;
use super::tts_module::TtsModule;
use crate::base_modules::IModule;
use crate::base_modules::Module;
#[cfg(target_os = "linux")]
use crate::desktop::linux::linux_module;
use async_lock::RwLock;
use bevy::ecs::event::Event;
use bevy::ecs::event::EventWriter;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::ResMut;
use bevy::platform::collections::HashMap;
use bevy_egui::egui;
use core::panic;
use egui_file_dialog::FileDialog;
use egui_notify::Toasts;
use futures::executor;
use std::fmt::format;
use std::sync::Arc;
use vl_global::vl_config::ConfigError;
use vl_global::vl_config::ConfigManager;
use vl_global::vl_config::VlConfig;

#[derive(Event)]
pub enum ModuleManagerEvent {
    LoadModule(String),
}

#[derive(Resource)]
pub struct ModuleManager {
    pub config: Arc<RwLock<ConfigManager>>,
    pub file_dialog: Arc<RwLock<FileDialog>>,
    pub(super) toast: Toasts,
    pending_error_messages: Vec<String>,
    modules: HashMap<String, Module>,
    pub(crate) selected_device_module:
        Option<Arc<RwLock<dyn DeviceModule>>>,
    pub(crate) selected_tts_module:
        Option<Arc<RwLock<dyn TtsModule>>>,
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
        config
            .modify_and_save(|config| self._update_config(config))?;
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
    // #endregion

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

    pub fn show_configs(
        &mut self,
        ui: &mut egui::Ui,
        config: &mut VlConfig,
        module_event_w: &mut EventWriter<ModuleManagerEvent>,
        tokio: &mut ResMut<bevy_tokio_tasks::TokioTasksRuntime>,
    ) {
        let runtime = tokio.runtime();
        // Organize modules in a hashmap.
        let mut modules: HashMap<String, Vec<String>> =
            HashMap::new();

        for (key, module) in &self.modules {
            let mut list = modules.get_mut(key);
            if list.is_none() {
                modules.insert(key.clone(), vec![]);
                list = modules.get_mut(key);
            }
            let list = list.unwrap();
            list.push(module.get_screen_name().to_string());
        }

        // Draw the options
        for (module_type, alternatives) in modules {
            let selected_module =
                config.selected_modules.get(&module_type);
            if selected_module.is_none() {
                continue;
            }
            let mut selected_module = selected_module.unwrap();

            ui.label(format!(
                "{module_type} Selecionado: {selected_module}"
            ));
            for linker_option in &alternatives {
                let text = linker_option.clone();
                ui.radio_value(
                    &mut selected_module,
                    linker_option,
                    text,
                );
            }
            // Update the config with the selected linker
            config
                .selected_modules
                .insert(module_type, selected_module.to_string());
        }

        // Add start modules buttons
        if let Some(module) = self.selected_tts_module.clone() {
            let mut module = executor::block_on(module.write());
            let module_type = module.get_module_type();
            if ui.button(format!("Iniciar {module_type}")).clicked() {
                if let Err(e) = runtime.block_on(module.start()) {
                    self.error(format!(
                        "Failed to start {module_type}!",
                    ));
                    log::error!("{e}");
                }
            }
        };
        if let Some(module) = self.selected_device_module.clone() {
            let mut module = executor::block_on(module.write());
            let module_type = module.get_module_type();
            if ui.button(format!("Iniciar {module_type}")).clicked() {
                if let Err(e) = runtime.block_on(module.start()) {
                    self.error(format!(
                        "Failed to start {module_type}!",
                    ));
                    log::error!("{e}");
                }
            }
        };
    }
}
