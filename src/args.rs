use clap::ArgAction;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Change into this directory
    #[arg(short = 'C', long)]
    pub path: Option<PathBuf>,
    /// Turn debugging information on
    #[arg(short, long, global = true, action(ArgAction::Count))]
    pub verbose: u8,
}
