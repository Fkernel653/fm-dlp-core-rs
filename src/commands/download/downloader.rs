use crate::commands::download::{config::DownloadConfig, params::DownloadParams};

pub struct Download {
    pub params: DownloadParams,
    pub conf: DownloadConfig,
}
