use std::sync::atomic::{AtomicBool, Ordering};

const RESET: &str = "\x1b[0m";

const BOLD_WHITE: &str = "\x1b[37m";
const GRAY: &str = "\x1b[90m";

const BOLD_RED: &str = "\x1b[1;31m";
const BOLD_GREEN: &str = "\x1b[1;32m";
const BOLD_YELLOW: &str = "\x1b[1;33m";
const BOLD_CYAN: &str = "\x1b[1;36m";

static COLORS_ENABLED: AtomicBool = AtomicBool::new(true);

/// Enables or disables colored output globally.
pub fn set_colors(enabled: bool) {
    COLORS_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Returns whether colored output is currently enabled.
fn colors_enabled() -> bool {
    COLORS_ENABLED.load(Ordering::Relaxed)
}

/// Applies an ANSI color code to the text if color output is enabled.
pub fn styled(text: &str, color: &str) -> String {
    if colors_enabled() {
        format!("{color}{text}{RESET}")
    } else {
        text.to_string()
    }
}

/// Formats text as a success message using the default prefix.
pub fn success(text: &str) -> String {
    success_with_prefix(text, "Success: ")
}

/// Formats text as a success message with a custom prefix (bold green).
pub fn success_with_prefix(text: &str, prefix: &str) -> String {
    if colors_enabled() {
        format!("{BOLD_GREEN}{prefix}{RESET}{text}")
    } else {
        format!("{prefix}{text}")
    }
}

/// Formats text as an error message using the default prefix.
pub fn error(text: &str) -> String {
    error_with_prefix(text, "Error: ")
}

/// Formats text as an error message with a custom prefix (bold red).
pub fn error_with_prefix(text: &str, prefix: &str) -> String {
    if colors_enabled() {
        format!("{BOLD_RED}{prefix}{RESET}{text}")
    } else {
        format!("{prefix}{text}")
    }
}

/// Formats text as an informational message using the default prefix.
pub fn info(text: &str) -> String {
    info_with_prefix(text, "Info: ")
}

/// Formats text as an informational message with a custom prefix (bold cyan).
pub fn info_with_prefix(text: &str, prefix: &str) -> String {
    if colors_enabled() {
        format!("{BOLD_CYAN}{prefix}{RESET}{text}")
    } else {
        format!("{prefix}{text}")
    }
}
