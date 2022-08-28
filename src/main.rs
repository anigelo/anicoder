mod lib;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    H264 {
        // Path to operate on, defaults to current directory
        #[clap(value_parser)]
        path: Option<String>
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::H264 { path } => {
            let path = parse_path_or_cd(path);
            lib::encode_to_h264(path);
        }
    }
}

fn parse_path_or_cd(path: &Option<String>) -> PathBuf {
    match path {
        Some(user_path) if PathBuf::from(&user_path).exists() => PathBuf::from(user_path),
        _ => std::env::current_dir().unwrap()
    }
}
