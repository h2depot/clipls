use std::{ffi::OsStr, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CopyMode {
    File,
    Text,
    Path,
}

#[derive(Parser, Debug)]
#[command(name = "clipls")]
#[command(
    disable_version_flag = true,
    about = "Select files from a directory and copy their contents"
)]
pub struct Args {
    /// Directory to browse.
    #[arg(default_value = ".", value_name = "PATH")]
    pub path: PathBuf,

    /// Files to copy to clipboard.
    #[arg(long = "fc", num_args = 1.., value_name = "FILES")]
    pub fastclip: Option<Vec<PathBuf>>,

    /// Include files in subdirectories.
    #[arg(short, long)]
    pub recursive: bool,
    /// Include hidden files and directories.
    #[arg(short = 'a', long = "all")]
    pub hidden: bool,
    /// Copy mode.
    #[arg(short = 'm', long = "mode", value_enum, default_value_t = CopyMode::File)]
    pub mode: CopyMode,
}

pub fn version_requested<I, S>(args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    args.into_iter().any(|arg| {
        let arg = arg.as_ref();
        arg == OsStr::new("-v") || arg == OsStr::new("--version")
    })
}

pub fn aboutme_requested<I, S>(args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    args.into_iter()
        .any(|arg| arg.as_ref() == OsStr::new("--aboutme"))
}

pub fn easteregg_requested<I, S>(args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    args.into_iter()
        .any(|arg| arg.as_ref() == OsStr::new("--easteregg"))
}

pub fn version() -> &'static str {
    include_str!("../../version/version.yaml")
        .lines()
        .find_map(|line| line.trim().strip_prefix("version:").map(str::trim))
        .filter(|version| !version.is_empty())
        .expect("version/version.yaml must contain a non-empty `version` field")
}
