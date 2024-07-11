extern crate walkdir;

use clap::Parser;
use flac::StreamReader;
use std::env;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;
use metaflac::Tag;

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
    println!("Finished!");
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

// fn print_metadata(file_path: &str) {
//     match StreamReader::<File>::from_file(file_path) {
//         Ok(stream) => {
//             // Copy of `StreamInfo` to help convert to a different audio format.
//             let info = stream.info();
//             // The explicit size for `Stream::iter` is the resulting decoded
//             // sample. You can usually find out the desired size of the
//             // samples with `info.bits_per_sample`.
//             if info.md5_sum == [0; 16] {
//                 println!("Empty MD5");
//             } else {
//                 dbg!(info);
//             }
//         }
//         Err(error) => println!("{:?}", error),
//     }
// }
fn print_metadata(file_path: &str) {
    let path = Path::new(file_path);
    match Tag::read_from_path(path) {
        Ok(tag) => {
            // Handling the case where `get_vorbis` returns an `Option<Vec<&str>>`
            let title = tag.get_vorbis("TITLE");
            match title {
                Some(titles) => {
                    let titles_vec: Vec<&str> = titles.collect();
                    if !titles_vec.is_empty() {
                        println!("Title: {}", titles_vec[0]);
                    } 

                }
                _ => println!("Title not found in metadata"),
            }


        }
        Err(e) => println!("Failed to read FLAC metadata for {}: {}", file_path, e),
    }
}