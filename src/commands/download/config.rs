use crate::commands::download::params::DownloadParams;
use crate::config::parameters::ParametersManager;
use toml::{Value, map::Map};

pub struct DownloadConfig {
    pub params: DownloadParams,
    pub parameters_manager: ParametersManager,
}

impl DownloadConfig {
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
                    eprintln!("Failed to load config: {e}");
                }
            }
        }

        if let Ok(Value::Table(current)) = Value::try_from(&self.params) {
            result.extend(current);
        }

        result
    }
}
