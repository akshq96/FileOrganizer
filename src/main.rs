//! A simple program to organize files in a folder into different categories.
//! 
//! The program will organize files into the following categories:
//! - Images
//! - Videos
//! - Music
//! - Documents
//! - Archives
//! - Applications

use std::{fs,io,path::Path, env};

fn main()-> io::Result<()> { 
    // Get folder path from command line argument or use default
    let folder_path = env::args().nth(1).unwrap_or_else(|| "./Downloads".to_string());
    
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

fn organize_files(folder_path: &str)-> io::Result<()> {
    let all_files = fs::read_dir(folder_path)?;
    for file in all_files {
        let file = file?;
        let path = file.path();
        if path.is_file() {
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();
            let target_folder = match extension.as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "ico" | "webp" => "Images",
                "mp4" | "mov" | "avi" | "mkv" | "webm" => "Videos",
                "mp3" | "wav" | "ogg" | "m4a" | "aac" => "Music",
                "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => "Documents",
                "zip" | "rar" | "7z" | "tar" | "gz" => "Archives",
                "exe" | "dmg" | "pkg" | "deb" | "rpm" => "Applications",
                _ => continue, // Skip files that don't match any category
            };
            let pathfornewfolder = Path::new(folder_path).join(target_folder);
            if !pathfornewfolder.exists() {
                fs::create_dir(&pathfornewfolder)?;
            }
            let file_name = path.file_name().unwrap();
            let new_location = pathfornewfolder.join(file_name);
            fs::rename(&path, &new_location)?;
            println!("Moved {} to {}", file_name.to_string_lossy(), target_folder);
        }
    }
    Ok(())
}
// Usage: cargo run [folder_path]
// Example: cargo run ./Downloads
// 
// The program will print the files that are moved to the console.
// 
// The program will exit with code 1 if the directory does not exist or is not a directory. 
