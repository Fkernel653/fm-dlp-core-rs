use std::error::Error;
use toml::{Value, map::Map};

use crate::{
    colors::success,
    commands::download::params::DownloadParams,
    config::{config_manager::ConfigManager, params::ConfigParams},
};

pub struct ParametersManager {
    pub params: ConfigParams,
    pub config_manager: ConfigManager,
    pub param_key: String,
}

impl ParametersManager {
    pub fn new(config_manager: ConfigManager, params: ConfigParams) -> Self {
        Self {
            config_manager,
            params,
            param_key: "parameters".to_string(),
        }
    }

    pub fn set_parameters(&self, params: &DownloadParams) -> Result<(), Box<dyn Error>> {
        let mut config = self.config_manager.load_config()?;
        let value = Value::try_from(params)?;
        config.insert(self.param_key.clone(), value);

        self.config_manager.update_config(&config)?;

        if !self.params.quiet {
            println!("{}", success("Parameters have been successfully saved"))
        }
        Ok(())
    }

    pub fn get_parameters(&self) -> Result<DownloadParams, Box<dyn Error>> {
        let config = self.config_manager.load_config()?;

        let value = config
            .get(&self.param_key)
            .cloned()
            .unwrap_or_else(|| Value::Table(Map::new()));

        Ok(value.try_into()?)
    }
}
