use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Organizes files in `folder_path` by extension-based categories.
pub fn organize_files(folder_path: &str) -> io::Result<()> {
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(target_folder) = categorize(&path) {
            move_file(folder_path, &path, target_folder)?;
        }
    }

    Ok(())
}

fn categorize(path: &Path) -> Option<&'static str> {
    let extension = path.extension()?.to_str()?.to_lowercase();
    match extension.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "ico" | "webp" => Some("Images"),
        "mp4" | "mov" | "avi" | "mkv" | "webm" => Some("Videos"),
        "mp3" | "wav" | "ogg" | "m4a" | "aac" => Some("Music"),
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => Some("Documents"),
        "zip" | "rar" | "7z" | "tar" | "gz" => Some("Archives"),
        "exe" | "dmg" | "pkg" | "deb" | "rpm" => Some("Applications"),
        _ => None,
    }
}

fn move_file(folder_path: &str, path: &Path, target_folder: &str) -> io::Result<()> {
    let target_dir = Path::new(folder_path).join(target_folder);
    if !target_dir.exists() {
        fs::create_dir(&target_dir)?;
    }

    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing file name"))?;

    let new_location: PathBuf = target_dir.join(file_name);
    fs::rename(path, &new_location)?;
    println!("Moved {} to {}", file_name.to_string_lossy(), target_folder);
    Ok(())
}
