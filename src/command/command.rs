use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "clipls")]
#[command(
    version,
    about = "Select files from a directory and copy their contents"
)]
pub struct Args {
    /// Directory to browse.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Include files in subdirectories.
    #[arg(short, long)]
    pub recursive: bool,

    /// Include hidden files and directories.
    #[arg(short = 'a', long = "all")]
    pub hidden: bool,
}
