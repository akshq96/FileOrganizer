use std::fs;
use tempfile::tempdir;

#[test]
fn moves_files_into_category_directories() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let base_path = temp_dir.path();

    create_file(base_path.join("photo.jpg"));
    create_file(base_path.join("video.mp4"));
    create_file(base_path.join("song.mp3"));
    create_file(base_path.join("slides.pptx"));
    create_file(base_path.join("archive.zip"));
    create_file(base_path.join("ignore.xyz"));

    rustchef::organize_files(base_path.to_str().unwrap()).expect("organize failed");

    assert!(base_path.join("Images/photo.jpg").exists());
    assert!(base_path.join("Videos/video.mp4").exists());
    assert!(base_path.join("Music/song.mp3").exists());
    assert!(base_path.join("Documents/slides.pptx").exists());
    assert!(base_path.join("Archives/archive.zip").exists());
    assert!(
        base_path.join("ignore.xyz").exists(),
        "Unsupported files should remain in place"
    );
}

fn create_file(path: impl AsRef<std::path::Path>) {
    fs::write(path, b"test data").expect("failed to write test file");
}
