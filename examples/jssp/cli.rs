use std::path::PathBuf;
use clap::Parser;
use log::error;


#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the data directory
    #[arg(short, long)]
    data_dir: Option<PathBuf>,

    /// Path to the single data file
    #[arg(short, long)]
    file: Option<PathBuf>,
}

pub fn parse_args() -> Args {
    let args = Args::parse();     
    if validate_args(&args).is_err() {
        error!("Validation of the cli args failed");
    }
    args
}

fn validate_args(args: &Args) -> Result<(), String> {
    // No validation is required for now
    Ok(())
}
