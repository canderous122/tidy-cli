use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Folder to tidy
    pub path: PathBuf,

    /// Print what would happen without moving anything
    #[arg(long)]
    pub dry_run: bool,
}
