pub(crate) mod device_module;
pub(crate) mod module_manager;
pub(crate) mod tts_module;

use std::any::type_name;
use std::sync::Arc;

use async_lock::RwLock;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::ResMut;
use bevy_tokio_tasks::TokioTasksRuntime;
use device_module::DeviceModule;
use futures::executor;
use futures::future::BoxFuture;
use std::fmt::Debug;
use tts_module::TtsModule;

use crate::base_modules::module_manager::{
    ModuleManager, ModuleManagerEvent,
};

#[derive(Debug, Clone)]
pub enum Module {
    TtsModule(Arc<RwLock<dyn TtsModule>>),
    DeviceModule(Arc<RwLock<dyn DeviceModule>>),
}

impl Module {
    fn get_module_type(&self) -> &'static str {
        {
            match self {
                Module::TtsModule(rw_lock) => {
                    let checks = executor::block_on(rw_lock.read());
                    checks.get_module_type()
                }
                Module::DeviceModule(rw_lock) => {
                    let checks = executor::block_on(rw_lock.read());
                    checks.get_module_type()
                }
            }
        }
    }
}

impl IModule for Module {
    fn is_started(&self) -> bool {
        match self {
            Module::TtsModule(rw_lock) => {
                let checks = executor::block_on(rw_lock.read());
                checks.is_started()
            }
            Module::DeviceModule(rw_lock) => {
                let checks = executor::block_on(rw_lock.read());
                checks.is_started()
            }
        }
    }

    fn start(
        &mut self,
    ) -> BoxFuture<Result<(), Box<dyn std::error::Error>>> {
        Box::pin(async move {
            match self {
                Module::TtsModule(rw_lock) => {
                    let mut checks = rw_lock.write().await;
                    checks.start().await?;
                }
                Module::DeviceModule(rw_lock) => {
                    let mut checks = rw_lock.write().await;
                    checks.start().await?;
                }
            }
            Ok(())
        })
    }

    fn get_screen_name(&self) -> &'static str {
        match self {
            Module::TtsModule(rw_lock) => {
                let checks = executor::block_on(rw_lock.read());
                checks.get_screen_name()
            }
            Module::DeviceModule(rw_lock) => {
                let checks = executor::block_on(rw_lock.read());
                checks.get_screen_name()
            }
        }
    }
    fn get_name() -> &'static str
    where
        Self: Sized,
    {
        panic!("This shall NOT be called")
    }
}

pub trait IModule: Debug + Send + Sync {
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
    let linker = result.selected_device_module.clone().unwrap();
    let mut linker = runtime.block_on(linker.write());
    let linker_start_result = runtime.block_on(linker.start());
    if let Err(e) = linker_start_result {
        log::error!("{e}");
        let error = "Linker did not start".to_owned();
        module_manager.error(error);
    }
}

pub fn module_manager_event_handler(
    mut module_manager: ResMut<ModuleManager>,
    mut event_r: EventReader<ModuleManagerEvent>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let runtime = runtime.runtime();
}
