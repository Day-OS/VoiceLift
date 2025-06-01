use thiserror::Error;
use vl_global::vl_config::ConfigError;

#[derive(Error, Debug)]
pub enum LinuxBackendError {
    #[error(
        "The Linux config section was not found in the config file."
    )]
    ConfigSectionNotFound,
    #[error("Config file error.")]
    ConfigError(#[from] ConfigError),
    #[error("Unknown Error")]
    UnknownError(#[from] anyhow::Error),
}
