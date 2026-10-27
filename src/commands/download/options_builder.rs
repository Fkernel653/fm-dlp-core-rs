use std::{any::Any, collections::HashMap, path::PathBuf};

use toml::{Value, map::Map};

use crate::{
    colors::{error, set_colors},
    commands::download::{config::DownloadConfig, params::DownloadParams},
};

pub struct OptionsBuilder {
    pub params: DownloadParams,
    pub applied: Map<String, Value>,
    pub yt_dlp: PathBuf,
}

impl OptionsBuilder {
    pub fn new(params: DownloadParams, config: DownloadConfig) -> Self {
        let applied = config.apply_config();
        set_colors(applied["color"].as_bool().unwrap_or(params.color));
        let yt_dlp = match which::which("yt-dlp") {
            Ok(path) => path,
            Err(_) => {
                eprintln!("{}", error("Failed to find yt-dlp in PATH!"));
                PathBuf::from(applied["yt_dlp_path"].to_string())
            }
        };

        Self {
            params,
            applied,
            yt_dlp,
        }
    }

    //pub fn build(&self) -> <HashMap<&'static str, Any>> {}

    pub fn parse_quality(&self) -> String {
        let q = self.applied["quality"].to_string();
        let sliced = &q[..q.len() - 1];

        if q == "best" {
            return "bestvideo".to_string();
        } else if q == "worst" {
            return "worstvideo".to_string();
        } else if q.parse::<i64>().is_ok() {
            return format!("bestvideo[height<={q}]");
        } else if q.ends_with("p") & sliced.parse::<i64>().is_ok() {
            return format!("bestvideo[height<={sliced}]");
        } else {
            return q;
        }
    }

    pub fn add_cookies(&self, mut opts: HashMap<&'static str, Box<dyn Any>>) {
        let Some(cookies) = self.applied.get("cookies") else {
            return;
        };
        let Some(c) = cookies.as_str() else {
            return;
        };
        if c.is_empty() {
            return;
        }
        let c_path = PathBuf::from(c);

        let v = Box::new(c_path.to_string_lossy().into_owned());

        if c_path.is_file() {
            opts.insert("cookiefile", v);
        } else {
            opts.insert("cookiesfrombrowser", v);
        }
    }
}
