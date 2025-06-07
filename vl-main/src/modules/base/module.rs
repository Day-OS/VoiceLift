use crate::modules::base::device_module::DeviceModule;
use crate::modules::base::i_module::IModule;
use crate::modules::base::tts_module::TtsModule;
use crate::ui::screens::Screen;
use async_lock::RwLock;
use futures::executor;
use futures::future::BoxFuture;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Module {
    TtsModule(Arc<RwLock<dyn TtsModule>>),
    DeviceModule(Arc<RwLock<dyn DeviceModule>>),
}

impl Module {
    pub fn get_module_type(&self) -> &'static str {
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
