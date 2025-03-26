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
use std::fs::{self, File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
pub struct IOEngine;

impl IOEngine {
    pub fn file_exists(filename: &str) -> bool {
        fs::metadata(filename).is_ok()
    }

    pub fn create_file(filename: &str) -> Result<bool> {
        if Self::file_exists(filename) {
            return Ok(false); // File already exists
        }

        OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .map(|_| true) // If successful, return true
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create file: {}", e))) // it propagate errors
    }

    /// this function appends a page of 4kb (4096 bytes) in our file.
    pub fn add_page(filename: &str, data: &[u8;4096]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(filename)?;

        file.write_all(data)?;
        file.flush()?; // Ensure data is written to disk
        Ok(())
    }

    //This function reads a specific page from a given file and the page number.
    pub fn read_page(filename: &str, page_number: u64) -> Result<[u8; 4096]> {
        let mut file = File::open(filename)?;

        let offset = page_number * 4096; //it calculates the offset considering it 0 index
        file.seek(SeekFrom::Start(offset))?; // moves to the correct page

        //making a fixed size array of 4kb and reading exact a page into it.
        let mut buffer: [u8; 4096] = [0; 4096];
        file.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn update_page(filename: &str, page_number: u64, data: &[u8; 4096]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .open(filename)?;
    
        let offset = page_number * 4096; //calculate offset 
        file.seek(SeekFrom::Start(offset))?; // move to the correct page
    
        file.write_all(data)?; // overwrite the page (only 4kbs not ahead part)
        file.flush()?; 
    
        Ok(())
    }
    pub fn delete_file(filename: &str) -> Result<()> {
        std::fs::remove_file(filename)?;
        Ok(())
    }
    
}
