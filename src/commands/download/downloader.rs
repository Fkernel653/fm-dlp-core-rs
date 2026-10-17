use futures::{StreamExt, executor::Enter};
use std::{collections::HashMap, path::Path};
use ytdown::{MediaInfo, Ytdown};

use crate::{
    colors::error,
    commands::download::{config::DownloadConfig, params::DownloadParams, parser::UrlParser},
};

pub const AUDIO_CODECS: [&str; 7] = ["mp3", "aac", "flac", "m4a", "opus", "vorbis", "wav"];
pub const VIDEO_CONTAINERS: [&str; 6] = ["mp4", "mov", "mkv", "webm", "avi", "flv"];

pub struct Download {
    pub params: DownloadParams,
    pub conf: DownloadConfig,
}

impl Download {
    async fn _download_url(&self) -> ytdown::Result<()> {
        let yt = Ytdown::builder().build()?;
        let parser = UrlParser::new(self.params.urls.clone(), self.params.quiet);

        for url in parser.parse() {
            let info = yt.resolve(&url).await?;

            match info {
                MediaInfo::Single(video) => {
                    let title = video.title.clone();
                    let fmt = video.formats().best_progressive()?;
                    yt.download(fmt, Path::new(&title)).await?;
                }

                MediaInfo::Collection(mut col) => {
                    while let Some(entry) = col.entries.next().await {
                        let entry = entry?;

                        if let MediaInfo::Single(video) = entry {
                            let Some(title) = &video.title else {
                                eprintln!("{}", error("No title found for video!"));
                                continue;
                            };
                            let fmt = video.formats().best_progressive()?;
                            yt.download(fmt, Path::new(title)).await?;
                        }
                    }
                }
            }
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
