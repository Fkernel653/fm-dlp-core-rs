use crate::{colors::success, config::params::ConfigParams};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::PathBuf};
use toml::{Value, map::Map};

pub struct ConfigDir {
    pub dirname: String,
}

impl ConfigDir {
    pub fn new() -> Self {
        Self {
            dirname: String::from("fm-dlp"),
        }
    }

    pub fn home_dir() -> Result<PathBuf, Box<dyn Error>> {
        home::home_dir().ok_or_else(|| "Could not find home directory".into())
    }

    pub fn base_dir() -> Result<PathBuf, Box<dyn Error>> {
        let home = Self::home_dir()?;

        use std::env;

        let dir = match env::consts::OS {
            "windows" => {
                let appdata = env::var("LOCALAPPDATA")
                    .or_else(|_| env::var("APPDATA"))
                    .unwrap_or_default();
                PathBuf::from(appdata)
            }
            "macos" => home.join("Library").join("Application Support"),
            _ => env::var_os("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| home.join(".config")),
        };

        Ok(dir)
    }

    pub fn path(&self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(Self::base_dir()?.join(&self.dirname))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigManager {
    pub params: ConfigParams,
    pub encoding: String,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            params: ConfigParams {
                quiet: false,
                color: true,
                config_file: ConfigDir::new()
                    .path()
                    .expect("failed to determine config directory"),
            },
            encoding: "utf-8".to_string(),
            extra: Map::new(),
        }
    }

    pub fn load_config(&self) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = &self.params.config_file;
        if !config.exists() {
            return Ok(Map::new());
        }
        let content = fs::read_to_string(config)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn update_config(&self, data: &Map<String, Value>) -> Result<String, Box<dyn Error>> {
        let toml_string = toml::to_string(&data)?;
        fs::write(&self.params.config_file, &toml_string)?;
        println!("{}", success("Configuration saved successfully"));
        Ok(toml_string)
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
