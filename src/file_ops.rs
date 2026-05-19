use walkdir::WalkDir;
use std::path::Path;

pub fn scan_directory(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    let valid_extensions = ["jpg", "jpeg", "png", "gif", "heic", "mp4", "mov"];
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            if let Some(ext) = entry.path().extension() {
                if valid_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                    files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    
    files
}

pub fn copy_file(source: &str, dest: &str) -> Result<(), std::io::Error> {
    std::fs::copy(source, dest)?;
    Ok(())
}

pub fn create_directory(path: &str) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(path)?;
    Ok(())
}