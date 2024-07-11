extern crate walkdir;

use std::env;
use std::path::Path;
use walkdir::WalkDir;
use claxon::FlacReader;

fn main() {
    // Get the directory path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];

    // Check if the provided path is a directory
    if !Path::new(dir_path).is_dir() {
        eprintln!("Error: {} is not a directory", dir_path);
        std::process::exit(1);
    }

    // Collect all FLAC files in the directory and its subdirectories
    let flac_files = find_flac_files(dir_path);

    // Print the metadata of FLAC files
    if flac_files.is_empty() {
        println!("No FLAC files found in the directory.");
    } else {
        println!("FLAC files found in the directory:");
        for file in flac_files {
            println!("Processing file: {}", file);
            match print_metadata(&file) {
                Ok(_) => (),
                Err(e) => eprintln!("Error reading metadata from {}: {}", file, e),
            }
        }
    }
}

fn find_flac_files(dir_path: &str) -> Vec<String> {
    let mut flac_files = Vec::new();

    // Recursively walk through the directory
    for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
        // Check if the entry is a file and has a .flac extension
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "flac" {
                    flac_files.push(entry.path().display().to_string());
                }
            }
        }
    }

    flac_files
}

fn print_metadata(file_path: &str) -> Result<(), String> {
    let reader = FlacReader::open(file_path).map_err(|e| e.to_string())?;
    let md5: [u8; 16] = reader.streaminfo().md5sum;
    if (md5.len() > 0) {
        dbg!("md5 {}", md5);

    } 
//    // Print all other tags
//     for (name, value) in reader.tags() {
//         println!("{}: {}", name, value);
//     }

    Ok(())
}
