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

    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,

    #[arg(short = 'd', long = "dry-run", default_value_t = false)]
    dry_run: bool,
}

//TODO Read about this trait!
#[derive(Clone)]
struct FileInfo {
    path: PathBuf,
    // name: String,
    modification_time: Option<SystemTime>,
}


fn display(content: &str) {
    println!("{}", content);
}


fn display_files(files: &Vec<FileInfo>) {
    for file in files {
        println!("{}",
            file.path.display()
        );
    }
}


fn get_files_in_directorry(path: &PathBuf) -> Vec<FileInfo> {
    let mut files: Vec<FileInfo> = Vec::new();

    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        // let name = entry.file_name().to_string_lossy().to_string();
                        let modification_time = metadata.modified().ok();
                        files.push(FileInfo {
                            path: entry.path(),
                            // name,
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

    files
}


fn delete_files(files: &Vec<FileInfo>, args: &Args) {
    for file in files {
        match fs::remove_file(&file.path) {
            Ok(_) => {
                if args.verbose {
                    println!("Deleted: {}", file.path.display());
                }
            },
            Err(e) => eprintln!("Failed to delete {}: {}", file.path.display(), e),
        }
    }
}


fn main() {
    let args = Args::parse();

    let path: PathBuf = args.path.clone().unwrap_or_else(|| {
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

    let mut files: Vec<FileInfo> = get_files_in_directorry(&path);

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

    // if args.verbose {
    //     display_files(&files);
    // }

    // for file in &files {
    //     println!(
    //         "{:<40} | Modified: {:?}",
    //         file.name,
    //         file.modification_time
    //     );
    // }

    //TODO: Handle case that argument number is equal or larger than number of files

    // let files_to_delete_new_2: Vec<FileInfo> = files.iter().take(args.number as usize).cloned().collect();
    // let files_to_delete: Vec<FileInfo> = files.iter().take(args.number as usize).cloned().collect();
    let mut files_to_delete: Vec<FileInfo> = files.clone();
    files_to_delete = files_to_delete.split_off(args.number as usize);


    if args.dry_run {
        display("Dry run; would delete files:");
        display_files(&files_to_delete);
    } else {
        delete_files(&files_to_delete, &args);
    }
}
