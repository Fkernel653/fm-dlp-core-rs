use std::path::PathBuf;

use crate::{
    colors::{error, set_colors, success},
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
    pub fn set_path(&self, path: String) -> bool {
        set_colors(self.params.color);
        let input_path = PathBuf::from(&path);

        if !input_path.is_dir() {
            panic!("{}", error("Please enter the correct path!"));
        }

        match self.config_manager.load_config() {
            Ok(mut map) => {
                map.insert(self.path_key.clone(), toml::Value::String(path.clone()));

                match self.config_manager.update_config(&map) {
                    Ok(_) => {
                        if !self.params.quiet {
                            println!("{}", success("Path saved successfully"));
                        }
                        true
                    }
                    Err(e) => {
                        panic!("{}", error(&format!("Failed to save config: {e}")));
                    }
                }
            }
            Err(e) => {
                panic!("{}", error(&format!("Failed to load config: {e}")));
            }
        }
    }

    pub fn get_path(&self) -> PathBuf {
        let home = ConfigDir::home_dir().unwrap_or_else(|_| PathBuf::from("."));

        match self.config_manager.load_config() {
            Ok(map) => match map.get(&self.path_key) {
                Some(toml::Value::String(s)) => PathBuf::from(s),
                _ => home,
            },
            Err(_) => home,
        }
    }
}
