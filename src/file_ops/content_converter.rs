use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

pub fn paths_as_text(files: &[PathBuf]) -> String {
    files
        .iter()
        .map(|file| file.display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn contents_as_text(files: &[PathBuf]) -> Result<String> {
    files
        .iter()
        .map(|file| {
            fs::read_to_string(file)
                .with_context(|| format!("failed to read file as UTF-8: {}", file.display()))
        })
        .collect::<Result<Vec<_>>>()
        .map(|contents| contents.join("\n"))
}
