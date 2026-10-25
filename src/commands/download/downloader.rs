use futures::stream::{self, StreamExt};
use std::{collections::HashMap, path::Path};
use ytdown::{MediaInfo, VideoInfo, Ytdown};

use crate::commands::download::{
    config::DownloadConfig, params::DownloadParams, parser::UrlParser,
};

pub const AUDIO_CODECS: [&str; 8] = ["mp3", "aac", "flac", "m4a", "opus", "vorbis", "wav", "alac"];
pub const VIDEO_CONTAINERS: [&str; 6] = ["mp4", "mov", "mkv", "webm", "avi", "flv"];

pub struct Download {
    pub params: DownloadParams,
    pub conf: DownloadConfig,
}

impl Download {
    async fn _download_one(yt: &Ytdown, video: VideoInfo) -> ytdown::Result<()> {
        let title = video.title.clone();
        let fmt = video.formats().best_progressive()?;
        yt.download(fmt, Path::new(&title)).await?;
        Ok(())
    }

    pub async fn download(&self) -> ytdown::Result<()> {
        let yt = Ytdown::builder().build()?;
        let parser = UrlParser::new(self.params.urls.clone(), self.params.quiet);

        let mut videos = Vec::new();

        for url in parser.parse() {
            let info = yt.resolve(&url).await?;
            match info {
                MediaInfo::Collection(mut col) => {
                    while let Some(entry) = col.entries.next().await {
                        let entry = entry?;
                        if let MediaInfo::Single(video) = yt.resolve(&entry.url).await? {
                            videos.push(video);
                        }
                    }
                }
                MediaInfo::Single(video) => videos.push(video),
            }
        }
        stream::iter(videos)
            .map(|video| Self::_download_one(&yt, video))
            .buffer_unordered(3)
            .for_each(|res| async move {
                if let Err(e) = res {
                    eprintln!("Download error: {e}");
                }
            })
            .await;

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
