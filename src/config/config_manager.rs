use crate::{
    colors::{error, set_colors},
    config::params::ConfigParams,
};
use std::{error::Error, fs, path::PathBuf};
use toml::{Value, map::Map};

pub struct ConfigDir {
    pub dirname: String,
    pub color: bool,
}

impl ConfigDir {
    pub fn new(color: bool) -> Self {
        Self {
            dirname: String::from("fm-dlp"),
            color,
        }
    }

    pub fn home_dir() -> Result<PathBuf, Box<dyn Error>> {
        home::home_dir().ok_or_else(|| "Failed to find home directory".into())
    }

    pub fn config_dir(&self) -> PathBuf {
        match Self::home_dir() {
            Ok(home) => {
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

                dir
            }
            Err(e) => {
                set_colors(self.color);
                panic!("{}", error(&format!("{e}")));
            }
        }
    }

    pub fn path(&self) -> PathBuf {
        self.config_dir().join(&self.dirname)
    }
}

pub struct ConfigManager {
    pub config_file: PathBuf,
}

impl ConfigManager {
    pub fn new(params: ConfigParams) -> Self {
        Self {
            config_file: params.config_file,
        }
    }

    pub fn update_config(&self, data: &Map<String, Value>) -> Result<String, Box<dyn Error>> {
        let toml_string = toml::to_string(&data)?;
        fs::write(&self.config_file, &toml_string)?;
        Ok(toml_string)
    }

    pub fn load_config(&self) -> Result<Map<String, Value>, Box<dyn Error>> {
        let config = &self.config_file;
        if !config.exists() {
            return Ok(Map::new());
        }
        let content = fs::read_to_string(config)?;
        Ok(toml::from_str(&content)?)
    }
}
