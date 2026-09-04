use std::path::PathBuf;

use anyhow::Result;

use crate::{
    clipboard,
    command::CopyMode,
    file_ops::{collect_listed_files, contents_as_text, paths_as_text},
};

pub fn run(fastclip_files: &[PathBuf], mode: CopyMode) -> Result<()> {
    let files = collect_listed_files(fastclip_files)?;
    match mode {
        CopyMode::File => clipboard::set_file_list(&files),
        CopyMode::Path => clipboard::set_text(&paths_as_text(&files)),
        CopyMode::Text => clipboard::set_text(&contents_as_text(&files)?),
    }?;

    println!("Copied {} file(s) to the clipboard.", files.len());
    Ok(())
}
