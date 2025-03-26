use crate::enums::db_error_status::DatabaseStatus;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Checks if a database (directory) exists.
///
/// # Arguments
/// * `database_name` - The name of the database (directory).
///
/// # Returns
/// * `true` if the database exists, otherwise `false`.
///
pub fn db_exists(database_name: &str) -> bool {
    Path::new(database_name).is_dir()
}

/// Checks if a specific file exists.
///
/// # Arguments
/// * `file_name` - The name of the file to check.
///
/// # Returns
/// * `true` if the file exists, otherwise `false`.
///
pub fn file_exists(file_name: &str) -> bool {
    fs::metadata(file_name).is_ok()
}

/// Determines the existence status of a file within a database.
///
/// # Arguments
/// * `database_name` - The name of the database (directory).
/// * `file_name` - The name of the file to check within the database.
///
/// # Returns
/// * `DatabaseFileStatus::DatabaseNotFound` if the database does not exist.
/// * `DatabaseFileStatus::FileNotFound` if the file does not exist but the database does.
/// * `DatabaseFileStatus::Exists` if both the database and file exist.
///
pub fn db_file_status(database_name: &str, file_name: &str) -> DatabaseStatus {
    let db_path: &Path = Path::new(database_name);
    let file_path: PathBuf = db_path.join(file_name);

    if !db_path.exists() {
        DatabaseStatus::DatabaseNotFound
    } else if !file_path.exists() {
        DatabaseStatus::FileNotFound
    } else {
        DatabaseStatus::FileExistsInDatabase
    }
}

/// Retrieves the size of a file in bytes.
///
/// # Parameters
/// - `path`: A reference to a `PathBuf` representing the file path.
///
/// # Returns
/// - `Ok(u64)`: The size of the file in bytes if the file exists.
/// - `Err(io::Error)`: An error if the file does not exist or cannot be accessed.
///
pub fn get_file_size(path: &Path) -> Result<u64, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Ensures that the specified file exists within the given database.
///
/// # Parameters
/// - `database_name`: The name of the database.
/// - `file_name`: The name of the file within the database.
///
/// # Returns
/// - `Ok(PathBuf)`: The path to the file if it exists.
/// - `Err(io::Error)`: An error if the file does not exist.
///
pub fn ensure_file_exists(database_name: &str, file_name: &str) -> Result<PathBuf, Error> {
    let file_exists = db_file_status(database_name, file_name);
    if let DatabaseStatus::FileExistsInDatabase = file_exists {
        Ok(Path::new(database_name).join(file_name))
    } else {
        Err(Error::new(ErrorKind::NotFound, file_exists.message()))
    }
}
