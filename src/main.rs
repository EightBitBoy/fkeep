use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;

//TODO Provide some explanations for the application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u64))]
    number: u64,

    #[arg(value_parser = clap::value_parser!(PathBuf))]
    path: Option<PathBuf>,

    //TODO Implement dry run option and logic!
}

fn main() {
    let args = Args::parse();

    let path = args.path.unwrap_or_else(|| {
        env::current_dir().expect("Error: Failed to get current directory.")
    });

    if !path.is_dir() {
        eprintln!("Error: The provided path is not a directory: {:?}", path.display());
        std::process::exit(1);
    }

    println!("Hello, {}!", args.number);
    println!("Using path: {:?}", path.display());

    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    println!("File: {:?}", entry_path.display());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
        }
    }
}
