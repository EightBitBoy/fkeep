use std::path::PathBuf;
use std::env;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u64))]
    number: u64,

    #[arg(value_parser = clap::value_parser!(PathBuf))]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.number);

    let path = args.path.unwrap_or_else(|| {
        env::current_dir().expect("Failed to get current directory")
    });

    //TODO: Make sure path is a dir!

    println!("Hello, {}!", args.number);
    println!("Using path: {:?}", path.display());
}
