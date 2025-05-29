pub(crate) mod device_manager;
pub(crate) mod module_manager;
pub(crate) mod tts_manager;

use std::any::type_name;
use std::sync::Arc;

use async_lock::RwLock;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::ResMut;
use bevy::platform::collections::HashMap;
use bevy_egui::egui;
use bevy_tokio_tasks::TokioTasksRuntime;
use device_manager::DeviceManager;
use egui_file_dialog::FileDialog;
use egui_notify::Toasts;
use futures::executor;
use futures::future::BoxFuture;
use paste::paste;
use std::fmt::Debug;
use tts_manager::TtsManager;
use vl_global::vl_config::ConfigManager;
use vl_global::vl_config::VlConfig;

use crate::base_managers::module_manager::ModuleManager;
#[cfg(target_os = "linux")]
use crate::desktop::linux::linux_module;

pub trait Module: Debug + Send + Sync {
    fn is_started(&self) -> bool;

    // Initialize a module
    fn start(
        &mut self,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>>;

    fn get_screen_name(&self) -> &'static str {
        let full_name = type_name::<Self>();
        match full_name.rsplit("::").next() {
            Some(name) => name,
            None => full_name,
        }
    }
    fn get_name() -> &'static str
    where
        Self: Sized,
    {
        let full_name = type_name::<Self>();
        match full_name.rsplit("::").next() {
            Some(name) => name,
            None => full_name,
        }
    }
}

pub fn initialize_module_manager(
    mut module_manager: ResMut<ModuleManager>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let runtime = runtime.runtime();
    let result = runtime.block_on(module_manager.initialize());
    let linker = result.selected_device_manager.clone().unwrap();
    let mut linker = runtime.block_on(linker.write());
    let linker_start_result = runtime.block_on(linker.start());
    if let Err(e) = linker_start_result {
        log::error!("{e}");
        let error = "Linker did not start".to_owned();
        module_manager.error(error);
    }
}
