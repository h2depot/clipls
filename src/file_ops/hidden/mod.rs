use std::{fs, fs::Metadata, path::Path};

use anyhow::{Context, Result};

#[cfg(target_family = "unix")]
#[path = "unix.rs"]
mod platform;
#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform;

pub(super) fn is_hidden_with_metadata(path: &Path, metadata: &Metadata) -> bool {
    platform::is_hidden(path, metadata)
}

pub fn is_hidden(path: &Path) -> Result<bool> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("failed to read metadata: {}", path.display()))?;
    Ok(is_hidden_with_metadata(path, &metadata))
}
