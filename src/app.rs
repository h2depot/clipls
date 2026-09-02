use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::{
    clipboard,
    command::{
        Args, CopyMode,
        ratatui::{PickerItem, plot},
    },
    file_ops::{collect_files, is_hidden},
};

pub fn run(args: Args) -> Result<()> {
    let files = collect_files(&args.path, args.recursive, args.hidden)?;

    if files.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    let picker_items = files
        .iter()
        .map(|file| {
            Ok(PickerItem {
                label: file
                    .strip_prefix(&args.path)
                    .unwrap_or(file)
                    .display()
                    .to_string(),
                is_directory: file.is_dir(),
                is_hidden: args.hidden && is_hidden(file)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    //Plot with ratatui by plot function.
    let selected = plot(&picker_items)?;
    let selected_files: Vec<_> = selected
        .into_iter()
        .filter_map(|index| files.get(index).cloned())
        .collect();

    if selected_files.is_empty() {
        println!("No files selected.");
        return Ok(());
    }

    match args.mode {
        CopyMode::File => clipboard::set_file_list(&selected_files),
        CopyMode::Path => clipboard::set_text(&paths_as_text(&selected_files)),
        CopyMode::Text => clipboard::set_text(&contents_as_text(&selected_files)?),
    }?;

    println!("Copied {} file(s) to the clipboard.", selected_files.len());
    Ok(())
}

fn paths_as_text(files: &[PathBuf]) -> String {
    files
        .iter()
        .map(|file| file.display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn contents_as_text(files: &[PathBuf]) -> Result<String> {
    files
        .iter()
        .map(|file| {
            fs::read_to_string(file)
                .with_context(|| format!("failed to read file as UTF-8: {}", file.display()))
        })
        .collect::<Result<Vec<_>>>()
        .map(|contents| contents.join("\n"))
}
