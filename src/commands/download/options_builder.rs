use crate::{
    colors::set_colors,
    commands::download::{config::DownloadConfig, params::DownloadParams},
};
use std::{any::Any, collections::HashMap, path::PathBuf};
use toml::{Value, map::Map};

pub struct OptionsBuilder {
    pub params: DownloadParams,
    pub applied: Map<String, Value>,
}

impl OptionsBuilder {
    pub fn new(params: DownloadParams, config: DownloadConfig) -> Self {
        let applied = config.apply_config();
        set_colors(applied["color"].as_bool().unwrap_or(params.color));

        Self { params, applied }
    }

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

    pub fn add_subtitles(&self, mut opts: HashMap<&'static str, Box<dyn Any>>) {
        let auto = self.applied["auto_subs"]
            .as_bool()
            .unwrap_or(self.params.auto_subs);
        if auto {
            opts.insert("writeautomaticsub", Box::new(auto));
            return;
        }

        let sub = self.applied["subtitles"]
            .as_bool()
            .unwrap_or(self.params.subtitles);
        if sub {
            let langs: Vec<String> = self.applied["subtitle_langs"]
                .as_str()
                .unwrap_or("")
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();

            opts.insert("writesubtitles", Box::new(true));
            opts.insert("subtitleslangs", Box::new(langs));

            if self.applied["embed_subs"]
                .as_bool()
                .unwrap_or(self.params.embed_subs)
            {}
        };
    }
}
