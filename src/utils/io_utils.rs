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

pub fn create_file(path: &Path) -> Result<(), String> {
    File::create(path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn create_dir(path: &Path) -> Result<(), String> {
    fs::create_dir(path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn remove_file(path: &Path) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn remove_dir(path: &Path) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| e.to_string())?;
    Ok(())
}
