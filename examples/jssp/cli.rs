use clap::Parser;
use log::error;
use std::path::PathBuf;

/// Jssp instance solver
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Path to the single data file. This option must be specified either as a cli param
    /// or in config file.
    #[arg(short = 'f', long = "input-file")]
    pub input_file: Option<PathBuf>,

    /// Output file name. This option must be specified either as a cli param
    /// or in config file.
    #[arg(short = 'o', long = "output-dir")]
    pub output_dir: Option<PathBuf>,

    /// Optional number of generations. If not specified implementation
    /// will provide reasonable default
    #[arg(long = "gens")]
    pub n_gen: Option<usize>,

    /// Optional population size. If not specified implemntation
    /// will provide reasonable default
    pub pop_size: Option<usize>,

    /// Path to config file with solver's parameters
    #[arg(short = 'c', long = "config")]
    pub cfg_file: Option<PathBuf>,
}

fn validate_args(args: &Args) -> Result<(), String> {
    if let Some(ref file) = args.input_file {
        if !file.is_file() {
            return Err("Specified data input file does not exist or is not a file".to_owned());
        }
    }
    if let Some(ref file) = args.cfg_file {
        if !file.is_file() {
            return Err("Specified config file does not exist or is not a file".to_owned());
        }
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
