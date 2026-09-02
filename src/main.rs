mod app;
mod command;
mod file_ops;

use anyhow::Result;
use clap::Parser;
use command::Args;

fn main() -> Result<()> {
    app::run(Args::parse())
}
