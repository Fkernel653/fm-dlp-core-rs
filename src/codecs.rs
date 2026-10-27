pub const AUDIO_CODECS: [&str; 8] = ["mp3", "aac", "alac", "m4a", "wav", "flac", "vorbis", "opus"];
pub const VIDEO_CONTAINERS: [&str; 6] = ["avi", "flv", "mp4", "mov", "mkv", "webm"];

pub fn get_audio_codecs() -> Vec<String> {
    AUDIO_CODECS.iter().map(|s| s.to_string()).collect()
}

pub fn get_video_containers() -> Vec<String> {
    VIDEO_CONTAINERS.iter().map(|s| s.to_string()).collect()
}

pub fn get_container_audio_map() -> Vec<(&'static str, String)> {
    vec![
        ("avi", "mp3".to_string()),
        ("flv", "aac".to_string()),
        ("mp4", "m4a".to_string()),
        ("mov", "m4a".to_string()),
        ("mkv", "opus".to_string()),
        ("webm", "opus".to_string()),
    ]
}
