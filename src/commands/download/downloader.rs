use std::collections::HashMap;
use ytdown::Ytdown;

use crate::commands::download::{
    config::DownloadConfig, params::DownloadParams, parser::UrlParser,
};

pub const AUDIO_CODECS: [&str; 7] = ["mp3", "aac", "flac", "m4a", "opus", "vorbis", "wav"];
pub const VIDEO_CONTAINERS: [&str; 6] = ["mp4", "mov", "mkv", "webm", "avi", "flv"];

pub struct Download {
    pub params: DownloadParams,
    pub conf: DownloadConfig,
}

impl Download {
    async fn _download(&self) -> ytdown::Result<()> {
        let yt = Ytdown::builder().build()?;
        for url in UrlParser::new(self.params.urls, self.params.quiet).into_iter() {
            let info = yt.resolve(url);
            // ...
        }
        Ok(())
    }

    pub fn get_audio_codecs() -> Vec<String> {
        AUDIO_CODECS.iter().map(|s| s.to_string()).collect()
    }

    pub fn get_video_containers() -> Vec<String> {
        VIDEO_CONTAINERS.iter().map(|s| s.to_string()).collect()
    }

    pub fn get_container_audio_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("mp4".to_string(), "m4a".to_string());
        map.insert("mov".to_string(), "m4a".to_string());
        map.insert("mkv".to_string(), "opus".to_string());
        map.insert("webm".to_string(), "opus".to_string());
        map.insert("avi".to_string(), "mp3".to_string());
        map.insert("flv".to_string(), "aac".to_string());
        map
    }
}
