use std::path::PathBuf;

use anyhow::Result;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod platform;
#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod platform;

/// Places files and directories on the operating system's clipboard.
pub fn set_file_list(files: &[PathBuf]) -> Result<()> {
    platform::set_file_list(files)
}

/// Places UTF-8 text on the operating system's clipboard.
pub fn set_text(text: &str) -> Result<()> {
    platform::set_text(text)
}
