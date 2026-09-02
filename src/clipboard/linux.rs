use std::path::PathBuf;

use anyhow::{Context, Result};

pub(super) fn set_file_list(files: &[PathBuf]) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new().context("failed to access the clipboard")?;
    clipboard
        .set()
        .file_list(files)
        .context("failed to copy files to the clipboard")
}

pub(super) fn set_text(text: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new().context("failed to access the clipboard")?;
    clipboard
        .set_text(text)
        .context("failed to copy text to the clipboard")
}
