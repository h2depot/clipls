mod app;
mod clipboard;
mod command;
mod file_ops;

use anyhow::Result;
use clap::Parser;
use command::{
    Args, aboutme_requested, easteregg_requested, ratatui::plot_aboutme, ratatui::plot_easteregg,
    version, version_requested,
};

fn main() -> Result<()> {
    let raw_args: Vec<_> = std::env::args_os().skip(1).collect();

    if version_requested(&raw_args) {
        println!("clipls {}", version());
        return Ok(());
    }

    if aboutme_requested(&raw_args) {
        plot_aboutme()?;
        return Ok(());
    }

    if easteregg_requested(&raw_args) {
        plot_easteregg()?;
        return Ok(());
    }

    app::run(Args::parse())
}
