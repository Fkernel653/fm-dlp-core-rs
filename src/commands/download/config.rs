use crate::{
    colors::{error, set_colors},
    commands::download::params::DownloadParams,
    config::parameters::ParametersManager,
};
use toml::{Value, map::Map};

pub struct DownloadConfig {
    pub params: DownloadParams,
    pub parameters_manager: ParametersManager,
}

impl DownloadConfig {
    pub fn save_config(&self) -> bool {
        if self.params.save {
            match self.parameters_manager.set_parameters(&self.params) {
                Ok(_) => true,
                Err(e) => {
                    set_colors(self.params.color);
                    panic!("{}", error(&format!("Failed to save config: {e}")))
                }
            }
        } else {
            false
        }
    }
    pub fn apply_config(&self) -> Map<String, Value> {
        let mut result = Map::new();

        if self.params.use_config {
            match self.parameters_manager.get_parameters() {
                Ok(saved) => {
                    if let Ok(Value::Table(table)) = Value::try_from(&saved) {
                        result.extend(table);
                    }
                }
                Err(e) => {
                    set_colors(self.params.color);
                    panic!("{}", error(&format!("Failed to load config: {e}")));
                }
            }
        }

        if let Ok(Value::Table(current)) = Value::try_from(&self.params) {
            result.extend(current);
        }

        result
    }
}
