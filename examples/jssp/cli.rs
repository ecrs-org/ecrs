use clap::Parser;
use log::error;
use std::path::PathBuf;

/// Jssp instance solver
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Path to the data directory
    #[arg(short = 'i', long = "input-dir")]
    pub input_dir: Option<PathBuf>,

    /// Path to the single data file
    #[arg(short = 'f', long = "input-file")]
    pub input_file: Option<PathBuf>,

    /// Output data directory
    #[arg(long = "output-dir")]
    pub output_dir: Option<PathBuf>,

    /// Output file name
    #[arg(short = 'o', long = "output-file")]
    pub output_file: Option<PathBuf>,
}

fn validate_args(args: &Args) -> Result<(), String> {
    let cloned_args = args.clone();
    if args.input_file.is_some() && !cloned_args.input_file.unwrap().is_file() {
        return Err("Specified data input file does not exist or is not a file".to_owned());
    }
    if args.input_dir.is_some() && !cloned_args.input_dir.unwrap().is_dir() {
        return Err("Specified data input directory does not exist or is not a directory".to_owned());
    }
    if args.output_dir.is_some() && !cloned_args.output_dir.unwrap().is_dir() {
        return Err("Specified data output directory does not exist or is not a directory".to_owned());
    }
    if args.output_file.is_some() && !cloned_args.output_file.unwrap().is_file() {
        return Err("Specified data output file does not exist or is not a directory".to_owned());
    }
    Ok(())
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    if let Err(err) = validate_args(&args) {
        error!("Validation of the cli args failed with error: {err}");
        panic!("Validation of the cli args failed with error: {err}");
    }
    args
}
