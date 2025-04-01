// file_io.rs - Handles low-level file I/O operations for the storage engine.
//
// This file contains essential functions for reading and writing pages
// to disk, ensuring efficient and reliable data persistence. It provides
// abstractions over system file operations, including handling page-aligned
// reads/writes, flushing data, and managing file descriptors.
//
// File I/O is a critical component of the storage layer, interacting
// closely with the buffer pool and WAL (Write-Ahead Logging) system.
//

use crate::enums::db_status::DatabaseStatus;
use crate::utils::fs_utils::{db_exists, db_file_status, ensure_file_exists, get_file_size};
use std::fs::{self, File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use crate::configs::config::Config::PAGE_SIZE;

pub struct IOEngine;

impl IOEngine {
    /// Creates a new database by creating a directory with the given name.1
    pub fn create_database(database_name: &str) -> Result<()> {
        if db_exists(database_name) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                DatabaseStatus::DatabaseAlreadyExists.message(),
            ));
        }
        fs::create_dir(database_name)?;
        Ok(())
    }

    /// Creates a new file inside the specified database.
    pub fn create_file(database_name: &str, file_name: &str) -> Result<()> {
        let file_exists: DatabaseStatus = db_file_status(database_name, file_name);
        if let DatabaseStatus::FileNotFound = file_exists {
            let path: PathBuf = Path::new(database_name).join(file_name);
            File::create(path)?;
            return Ok(());
        } else {
            return Err(Error::new(ErrorKind::AlreadyExists, file_exists.message()));
        }
    }

    /// This function appends a page of 4kb (4096 bytes) in our file.
    pub fn add_page(database_name: &str, file_name: &str, data: &[u8; PAGE_SIZE as usize]) -> Result<()> {
        let path: PathBuf = ensure_file_exists(database_name, file_name)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(false)
            .append(true)
            .open(path)?;

        file.write_all(data)?;
        file.flush()?; // Ensure data is written to disk
        return Ok(());
    }

    /// Validates if the given page number is within the file's bounds.
    fn validate_page_bounds(path: &Path, page_number: u32) -> Result<u32> {
        let file_size = get_file_size(path)? as u32;
        let offset = page_number * PAGE_SIZE;
        if file_size < offset + PAGE_SIZE {
            return Err(Error::new(
                ErrorKind::NotFound,
                DatabaseStatus::PageNotFoundInFile.message(),
            ));
        }
        Ok(offset)
    }

    //This function reads a specific page from a given file and the page number.
    pub fn read_page(database_name: &str, file_name: &str, page_number: u32) -> Result<[u8; PAGE_SIZE as usize]> {
        let path: PathBuf = ensure_file_exists(database_name, file_name)?;
        let offset: u32 = Self::validate_page_bounds(&path, page_number)?;
        let mut file: File = File::open(path)?;

        file.seek(SeekFrom::Start(offset.into()))?; // moves to the correct page

        //making a fixed size array of 4kb and reading exact a page into it.
        let mut buffer: [u8; PAGE_SIZE as usize] = [0; PAGE_SIZE as usize];
        file.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    /// Updates a specific 4KB page in the file by overwriting it.
    pub fn update_page(
        database_name: &str,
        file_name: &str,
        page_number: u32,
        data: &[u8; PAGE_SIZE as usize],
    ) -> Result<()> {
        let path = ensure_file_exists(database_name, file_name)?;
        let offset= Self::validate_page_bounds(&path, page_number)?;

        let mut file: File = OpenOptions::new().write(true).open(path)?;
        file.seek(SeekFrom::Start(offset.into()))?; // Move to the correct page
        file.write_all(data)?; // Overwrite exactly 4KB
        file.flush()?;

        Ok(())
    }

    /// Deletes the specified file from the database.
    pub fn delete_file(database_name: &str, file_name: &str) -> Result<()> {
        let path = ensure_file_exists(database_name, file_name)?;
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Deletes the entire database directory and all its contents.
    pub fn delete_database(database_name: &str) -> Result<()> {
        if !db_exists(database_name) {
            return Err(Error::new(
                ErrorKind::NotFound,
                DatabaseStatus::DatabaseNotFound.message(),
            ));
        }
        fs::remove_dir_all(database_name)?; // Removes the directory and all its contents
        Ok(())
    }
}
