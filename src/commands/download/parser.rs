use std::collections::HashMap;
use std::path::Path;

pub const AUDIO_CODECS: [&str; 7] = ["mp3", "aac", "flac", "m4a", "opus", "vorbis", "wav"];
pub const VIDEO_CONTAINERS: [&str; 6] = ["mp4", "mov", "mkv", "webm", "avi", "flv"];

pub struct UrlParser {
    urls: String,
    quiet: bool,
}

impl UrlParser {
    pub fn new(urls: String, quiet: bool) -> Self {
        Self { urls, quiet }
    }

    pub fn parse(&self) -> Vec<String> {
        if self.urls.is_empty() {
            return Vec::new();
        }

        let url_path = Path::new(&self.urls);
        if url_path.is_file() {
            return self.parse_url_file(url_path);
        }

        self.urls
            .replace(',', " ")
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }

    fn parse_url_file(&self, file_path: &Path) -> Vec<String> {
        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading URL file: {}", e);
                std::process::exit(1);
            }
        };

        let mut urls = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                urls.extend(
                    trimmed
                        .replace(',', " ")
                        .split_whitespace()
                        .filter(|s| !s.is_empty())
                        .map(String::from),
                );
            }
        }

        if !self.quiet {
            println!("📁 Loaded {} URLs from file: {}", urls.len(), self.urls);
        }

        urls
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
