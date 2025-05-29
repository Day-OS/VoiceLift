use super::device_manager::DeviceManager;
use super::tts_manager::TtsManager;
use crate::base_managers::Module;
#[cfg(target_os = "linux")]
use crate::desktop::linux::linux_module;
use async_lock::RwLock;
use bevy::ecs::resource::Resource;
use bevy::platform::collections::HashMap;
use bevy_egui::egui;
use egui_file_dialog::FileDialog;
use egui_notify::Toasts;
use futures::executor;
use paste::paste;
use std::sync::Arc;
use vl_global::vl_config::ConfigManager;
use vl_global::vl_config::VlConfig;

#[derive(Resource)]
pub struct ModuleManager {
    pub config: Arc<RwLock<ConfigManager>>,
    pub file_dialog: Arc<RwLock<FileDialog>>,
    pub(super) toast: Toasts,
    pending_error_messages: Vec<String>,
    device_managers: HashMap<String, Arc<RwLock<dyn DeviceManager>>>,
    tts_managers: HashMap<String, Arc<RwLock<dyn TtsManager>>>,
    pub(super) selected_device_manager:
        Option<Arc<RwLock<dyn DeviceManager>>>,
    pub(super) selected_tts_manager:
        Option<Arc<RwLock<dyn TtsManager>>>,
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! define_modules_fns {
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

                pub fn [<does_ $field _exist>](&self, name: &String) -> bool {
                    let module = self.[<$field s>].get(name);
                    module.is_some()
                }

                pub fn [<get_ $field s>](&self,) -> Vec<String> {
                    self.[<$field s>].keys().map(|str| str.clone()).collect()
                }

                fn [<update_config_ $field>](&mut self, config: &mut VlConfig){
                    if let Some(module) = &mut config.[<selected_ $field>] {
                        let module: &String = module;
                        let does_this_module_exist: bool = self.[<does_ $field _exist>](module);
                        if does_this_module_exist{
                            self.[<selected_ $field>] = self.[<$field s>].get(module).cloned();
                            return;
                        }
                    }
                    if let Some((_, module)) = self.[<$field s>].iter().next() {
                        let module_lock = executor::block_on(module.read());
                        config.[<selected_ $field>] = Some(module_lock.get_screen_name().to_owned());

                    }
                }

                fn [<show_config_ $field>](&mut self, ui: &mut egui::Ui, config: &mut VlConfig){
                    if let Some(module) = &self.[<selected_ $field>]{
                        let alternatives = self.[<get_ $field s>]();
                        let module_type_name = executor::block_on(module.read()).get_module_type();
                        let mut selected =  config.[<selected_ $field>].clone();
                        if let Some(module_name) = &mut selected{
                            ui.label(format!("{module_type_name} Selecionado: {module_name}"));
                            for linker_option in alternatives{
                                ui.radio_value(module_name, linker_option.clone(), linker_option);
                            }
                        }
                        // Update the config with the selected linker
                        config.selected_device_linker = selected;
                    }


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

            pub fn update_config(&mut self, config: &mut VlConfig){
                $(
                    self.[<update_config_ $field>](config);
                )*

            }

            pub fn show_configs(&mut self, ui: &mut egui::Ui, config: &mut VlConfig){
                $(
                    self.[<show_config_ $field>](ui, config);
                )*

            }
        }
    };
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
            device_managers: HashMap::new(),
            tts_managers: HashMap::new(),
            selected_device_manager: None,
            selected_tts_manager: None,
        }
    }
    pub async fn initialize(&mut self) -> &mut Self {
        #[cfg(target_os = "linux")]
        {
            let module = linux_module::LinuxModule::new().await;
            let module_name = module.get_screen_name();
            let linux_module = Arc::new(RwLock::new(module));

            self.device_managers
                .insert(module_name.to_owned(), linux_module.clone());
            self.selected_device_manager = Some(linux_module.clone());

            self.tts_managers
                .insert(module_name.to_owned(), linux_module.clone());
            self.selected_tts_manager = Some(linux_module.clone())
        }

        // update config!
        let config_clone = self.config.clone();
        let mut config = executor::block_on(config_clone.write());
        config.modify_and_save(|config| self.update_config(config));

        self
    }
    define_modules_fns! {device_manager, tts_manager}

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
