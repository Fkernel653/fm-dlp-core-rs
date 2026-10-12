use std::path::PathBuf;

use crate::{
    colors::{error, success},
    config::{
        config_manager::{ConfigDir, ConfigManager},
        params::ConfigParams,
    },
};

pub struct PathManager {
    pub params: ConfigParams,
    pub config_manager: ConfigManager,
    pub path_key: String,
}

impl PathManager {
    pub fn set_path(&self, path: String) {
        let input_path = PathBuf::from(&path);

        if !input_path.is_dir() {
            eprintln!("{}", error("Please enter the correct path!"));
            return;
        }

        match self.config_manager.load_config() {
            Ok(mut map) => {
                map.insert(self.path_key.clone(), toml::Value::String(path.clone()));

                match self.config_manager.update_config(&map) {
                    Ok(_) => {
                        if !self.params.quiet {
                            println!("{}", success("Path saved successfully"));
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", error(&format!("Failed to save config: {}", e)));
                        return;
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", error(&format!("Failed to load config: {}", e)));
                return;
            }
        }
    }

    pub fn get_path(&self) -> PathBuf {
        match self.config_manager.load_config() {
            Ok(map) => match map.get(&self.path_key) {
                Some(toml::Value::String(s)) => PathBuf::from(s),
                _ => ConfigDir::home_dir().unwrap_or_else(|_| PathBuf::from(".")),
            },
            Err(_) => ConfigDir::home_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
}
