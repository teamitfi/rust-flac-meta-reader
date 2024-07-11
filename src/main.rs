extern crate walkdir;

use clap::Parser;
use flac::StreamReader;
use std::fs::File;
use walkdir::WalkDir;

#[derive(Parser)]
struct CliArguments {
    path: std::path::PathBuf,
}

fn main() {
    let args = CliArguments::parse();

    let dir_path = args.path;

    // Check if the provided path is a directory
    if !dir_path.is_dir() {
        eprintln!("Error: {} is not a directory", dir_path.display());
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
            print_metadata(&file);
        }
    }
}

fn find_flac_files(dir_path: std::path::PathBuf) -> Vec<String> {
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

fn print_metadata(file_path: &str) {
    match StreamReader::<File>::from_file(file_path) {
        Ok(stream) => {
            // Copy of `StreamInfo` to help convert to a different audio format.
            let info = stream.info();
            // The explicit size for `Stream::iter` is the resulting decoded
            // sample. You can usually find out the desired size of the
            // samples with `info.bits_per_sample`.
            dbg!(info);
            if info.md5_sum == [0; 16] {
                println!("Empty MD5");
            }
        }
        Err(error) => println!("{:?}", error),
    }
}
