use std::fs::{self, File};
use std::path::Path;

pub fn file_exists(path: &Path) -> bool {
    if path.exists() && path.is_file() {
        return true;
    }
    return false;
}

pub fn dir_exists(path: &Path) -> bool {
    if path.exists() && path.is_dir() {
        return true;
    }
    return false;
}

pub fn create_file(path: &Path) {
    File::create(path).unwrap();
}

pub fn create_dir(path: &Path) {
    fs::create_dir(path).unwrap()
}

pub fn remove_file(path: &Path) {
    fs::remove_file(path).unwrap();
}

pub fn remove_dir(path: &Path) {
    fs::remove_dir_all(path).unwrap();
}
