use walkdir::{WalkDir, DirEntry};
use std::path::{Path, PathBuf};

/// Recursively find files with a specific extension
pub fn find_files(root: &str, extension: &str) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_extension(e, extension))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Recursively find directories
pub fn find_dirs(root: &str) -> Vec<PathBuf> {
    WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// List immediate subdirectories (non-recursive)
pub fn list_immediate_subdirs(root: &str) -> Vec<PathBuf> {
    WalkDir::new(root)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn is_extension(entry: &DirEntry, ext: &str) -> bool {
    entry.path()
        .extension()
        .map(|e| e == ext)
        .unwrap_or(false)
}

/// Read file content, returning empty string on error
pub fn read_to_string_lossy(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|_| String::new())
}

/// Check if a path exists
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}
