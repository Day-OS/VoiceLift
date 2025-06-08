use crate::modules::base::device_module::DeviceModule;
use crate::modules::base::i_module::IModule;
use crate::modules::base::tts_module::TtsModule;
use async_lock::RwLock;
use busrt::async_trait;
use futures::executor;
use std::fmt::Debug;
use std::sync::Arc;

pub enum ModuleType {
    TtsModule,
    DeviceModule,
}

impl From<Module> for ModuleType {
    fn from(value: Module) -> Self {
        match value {
            Module::TtsModule(_) => Self::TtsModule,
            Module::DeviceModule(_) => Self::DeviceModule,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Module {
    TtsModule(Arc<RwLock<dyn TtsModule>>),
    DeviceModule(Arc<RwLock<dyn DeviceModule>>),
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        let type_check =
            self.get_module_type() == other.get_module_type();
        let id_check =
            self.get_screen_name() == other.get_screen_name();
        type_check && id_check
    }
}

impl From<Arc<RwLock<dyn TtsModule>>> for Module {
    fn from(value: Arc<RwLock<dyn TtsModule>>) -> Self {
        Self::TtsModule(value)
    }
}

impl From<Arc<RwLock<dyn DeviceModule>>> for Module {
    fn from(value: Arc<RwLock<dyn DeviceModule>>) -> Self {
        Self::DeviceModule(value)
    }
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
    pub fn is_module_type(&self, module_type: &ModuleType) -> bool {
        matches!(
            (self, module_type),
            (Module::TtsModule(_), ModuleType::TtsModule)
                | (Module::DeviceModule(_), ModuleType::DeviceModule)
        )
    }
}

#[async_trait]
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

    async fn start(&mut self) -> anyhow::Result<()> {
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
