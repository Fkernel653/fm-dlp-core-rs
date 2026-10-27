use std::path::Path;

use crate::colors::{error, info, set_colors};

pub struct UrlParser {
    pub urls: String,
    pub quiet: bool,
    pub color: bool,
}

impl UrlParser {
    pub fn new(urls: String, quiet: bool, color: bool) -> Self {
        Self { urls, quiet, color }
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
        set_colors(self.color);

        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                panic!("{}", error(&format!("Failed to read URL file: {e}")));
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
            println!(
                "{}",
                info(&format!(
                    "Loaded {} URLs from file: {}",
                    urls.len(),
                    self.urls
                ))
            );
        }

        urls
    }
}
