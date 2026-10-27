use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadParams {
    pub urls: String,
    pub codec: String,
    pub kbps: u32,
    pub quality: String,
    pub jobs: usize,
    pub quiet: bool,
    pub metadata: bool,
    pub keep: bool,
    pub save: bool,
    pub use_config: bool,
    pub config_file: Option<String>,
    pub path: Option<String>,
    pub only_video: bool,
    pub color: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ffmpeg_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub yt_dlp_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
}

pub enum Param<'a> {
    Str(&'a str),
    Int(u32),
    Bool(bool),
}

impl<'a> IntoIterator for &'a DownloadParams {
    type Item = (&'static str, Param<'a>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v: Vec<(&'static str, Param<'a>)> = vec![
            ("urls", Param::Str(&self.urls)),
            ("codec", Param::Str(&self.codec)),
            ("kbps", Param::Int(self.kbps)),
            ("quality", Param::Str(&self.quality)),
            ("jobs", Param::Int(self.jobs as u32)),
            ("quiet", Param::Bool(self.quiet)),
            ("metadata", Param::Bool(self.metadata)),
            ("keep", Param::Bool(self.keep)),
            ("save", Param::Bool(self.save)),
            ("use_config", Param::Bool(self.use_config)),
            ("only_video", Param::Bool(self.only_video)),
            ("color", Param::Bool(self.color)),
        ];

        if let Some(config_file) = &self.config_file {
            v.push(("config_file", Param::Str(config_file)));
        }
        if let Some(path) = &self.path {
            v.push(("path", Param::Str(path)));
        }
        if let Some(ffmpeg_path) = &self.ffmpeg_path {
            v.push(("ffmpeg_path", Param::Str(ffmpeg_path)));
        }
        if let Some(yt_dlp_path) = &self.yt_dlp_path {
            v.push(("yt_dlp_path", Param::Str(yt_dlp_path)));
        }
        if let Some(cookies) = &self.cookies {
            v.push(("cookies", Param::Str(cookies)));
        }
        if let Some(remote) = &self.remote {
            v.push(("remote", Param::Str(remote)));
        }

        v.into_iter()
    }
}
