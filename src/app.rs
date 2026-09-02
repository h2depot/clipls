use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{
    command::{Args, ratatui::plot},
    file_ops::collect_files,
};

pub fn run(args: Args) -> Result<()> {
    let files = collect_files(&args.path, args.recursive, args.hidden)?;

    if files.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    let labels: Vec<_> = files
        .iter()
        .map(|file| {
            file.strip_prefix(&args.path)
                .unwrap_or(file)
                .display()
                .to_string()
        })
        .collect();
    let selected = plot(&labels)?;
    let selected_files: Vec<_> = selected
        .into_iter()
        .filter_map(|index| files.get(index).cloned())
        .collect();

    // Clipboard output will replace this hand-off in the next step.
    print_file_list(&args.path, &selected_files);
    Ok(())
}

fn print_file_list(root: &Path, files: &[PathBuf]) {
    if files.is_empty() {
        println!("No files selected.");
        return;
    }

    for file in files {
        println!("{}", file.strip_prefix(root).unwrap_or(file).display());
    }
}
