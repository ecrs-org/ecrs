use clap::Parser;
use log::error;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Path to the data directory
    #[arg(short, long)]
    data_dir: Option<PathBuf>,

    /// Path to the single data file
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn validate_args(args: &Args) -> Result<(), String> {
    let cloned_args = args.clone();
    if args.file.is_some() {
        if !cloned_args.file.unwrap().is_file() {
            return Err("Specified data file does not exist or is not a file".to_owned());
        }
    }

    if args.data_dir.is_some() {
        if !cloned_args.data_dir.unwrap().is_dir() {
            return Err("Specified data directory does not exist or is not a directory".to_owned());
        }
    }
    Ok(())
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    if validate_args(&args).is_err() {
        error!("Validation of the cli args failed");
    }
    args
}
