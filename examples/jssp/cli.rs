use clap::Parser;
use log::error;
use std::path::PathBuf;

/// Jssp instance solver
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Path to the single data file
    #[arg(short = 'f', long = "input-file")]
    pub input_file: PathBuf,

    /// Output file name
    #[arg(short = 'o', long = "output-file")]
    pub output_file: PathBuf,
}

fn validate_args(args: &Args) -> Result<(), String> {
    if !args.input_file.is_file() {
        return Err("Specified data input file does not exist or is not a file".to_owned());
    }
    Ok(())
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    if let Err(err) = validate_args(&args) {
        panic!("Validation of the cli args failed with error: {err}");
    }
    args
}
