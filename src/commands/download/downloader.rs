use ytdown::Ytdown;

use crate::commands::download::{config::DownloadConfig, params::DownloadParams};

pub struct Download {
    pub params: DownloadParams,
    pub conf: DownloadConfig,
}

impl Download {
    #[tokio::main]
    async fn download(&self) -> ytdown::Result<()> {
        let _ = Ytdown::builder().build()?;
        Ok(())
    }
}
