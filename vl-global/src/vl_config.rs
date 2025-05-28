use config::Config;
use serde::Deserialize;
use serde::Serialize;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use homedir::my_home;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("data store disconnected")]
    HomeDirError(#[from] homedir::GetHomeError),
    #[error("Home dir does not exist")]
    HomeDirNotExist,
    #[error("Failed to serialize TOML")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("Failed to deserialize TOML")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("Error while trying to write or read to system")]
    IoError(#[from] std::io::Error),
    #[error("ConfigError")]
    ConfigError(#[from] config::ConfigError),
}

pub struct ConfigManager {
    settings: Config,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = Self::_get_config_path()?;

        if !config_path.exists() {
            // Create the parent directory if it doesn't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let config = VlConfig::default();
            Self::_save_vl_config(&config_path, &config)?;
        }
        let settings = Self::load_settings(&config_path);

        let mut config = Self {
            settings,
            config_path,
        };

        config.save()?;

        Ok(config)
    }

    fn load_settings(config_path: &Path) -> Config {
        Config::builder()
            .add_source(config::File::with_name(
                config_path.to_str().unwrap(),
            ))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap()
    }

    pub fn save(&mut self) -> Result<(), ConfigError> {
        self.modify_and_save(|_| {})
    }

    pub fn modify_and_save(
        &mut self,
        callback: fn(vl_config: &mut VlConfig),
    ) -> Result<(), ConfigError> {
        let mut config: VlConfig =
            self.settings.clone().try_deserialize()?;

        callback(&mut config); // Modify the configuration

        Self::_save_vl_config(&self.config_path, &config)?;

        // Reload settings
        self.settings = Self::load_settings(&self.config_path);
        Ok(())
    }

    fn _save_vl_config(
        config_path: &PathBuf,
        vl_config: &VlConfig,
    ) -> Result<(), ConfigError> {
        let toml_string = toml::to_string_pretty(&vl_config)?;

        // Write the TOML string to the configuration file
        let mut file = fs::File::create(config_path)?;
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }

    fn _get_config_path() -> Result<PathBuf, ConfigError> {
        let home = my_home()?.ok_or(ConfigError::HomeDirNotExist)?;

        let path = home
            .join(".config")
            .join("voice_lift")
            .join("config.toml");

        Ok(path)
        //home / ".config" / "voice_lift" / "config.toml"
    }
}

#[derive(
    Debug,
    // serde_derive::Deserialize,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Clone,
)]
pub struct VlConfig {
    linux: Option<LinuxConfig>,
}

impl Default for VlConfig {
    fn default() -> Self {
        VlConfig {
            linux: Some(LinuxConfig::default()),
        }
    }
}

#[derive(
    Debug,
    // serde_derive::Deserialize,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Clone,
    Default,
)]
pub struct LinuxConfig {
    piper_tts_model: String,
}
impl VlConfig {}

//"/usr/share/piper-voices/pt/pt_BR/droidela-v2/medium/droidela-v2.onnx.json",
