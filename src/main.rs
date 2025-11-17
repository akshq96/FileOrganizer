//! A simple program to organize files in a folder into different categories.
//!
//! The program will organize files into the following categories:
//! - Images
//! - Videos
//! - Music
//! - Documents
//! - Archives
//! - Applications

use rustchef::organize_files;
use std::{env, io, path::Path};

fn main() -> io::Result<()> {
    // Get folder path from command line argument or use default
    let folder_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "./Downloads".to_string());

    // Check if the directory exists
    let path = Path::new(&folder_path);
    if !path.exists() {
        eprintln!("Error: Directory '{}' does not exist!", folder_path);
        eprintln!("Usage: cargo run [folder_path]");
        eprintln!("Example: cargo run ./Downloads");
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory!", folder_path);
        std::process::exit(1);
    }

    println!("Organizing files in: {}", folder_path);
    organize_files(&folder_path)
}
// Usage: cargo run [folder_path]
// Example: cargo run ./Downloads
// The program will print the files that are moved to the console.
// The program will exit with code 1 if the directory does not exist or is not a directory.
