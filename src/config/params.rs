use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigParams {
    pub quiet: bool,
    pub color: bool,
    pub config_file: std::path::PathBuf,
}
