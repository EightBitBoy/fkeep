use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

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

struct FileInfo {
    path: PathBuf,
    name: String,
    modification_time: Option<SystemTime>,
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

    // println!("Hello, {}!", args.number);
    // println!("Using path: {:?}", path.display());

    // match fs::read_dir(&path) {
    //     Ok(entries) => {
    //         for entry in entries.flatten() {
    //             let entry_path = entry.path();
    //             if entry_path.is_file() {
    //                 println!("File: {:?}", entry_path.display());
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to read directory: {}", e);
    //     }
    // }

    let mut files: Vec<FileInfo> = Vec::new();

    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let modification_time = metadata.modified().ok();
                        files.push(FileInfo {
                            path: entry.path(),
                            name,
                            modification_time,
                        });
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            std::process::exit(1);
        }
    }

    // for file in &files {
    //     println!(
    //         "File: {:?}, Name: {}, Modification Time: {:?}",
    //         file.path.display(),
    //         file.name,
    //         file.modification_time
    //             .map(|t| t.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
    //             .unwrap_or(0)
    //     );
    // }

    // for file in &files {
    //     println!(
    //         "{:<40} | Modified: {:?}",
    //         file.name,
    //         file.modification_time
    //     );
    // }

    //oldest first
    files.sort_by(|a, b| {
        a.modification_time.cmp(&b.modification_time)
    });

    for file in &files {
        println!(
            "{:<40} | Modified: {:?}",
            file.name,
            file.modification_time
        );
    }

    let files_to_delete = &files[..args.number as usize];
    for file in files_to_delete {
        // println!("Deleting: {:?}", file.path.display());
        match fs::remove_file(&file.path) {
            Ok(_) => println!("Deleted: {}", file.path.display()),
            Err(e) => eprintln!("Failed to delete {}: {}", file.path.display(), e),
        }
    }
}
