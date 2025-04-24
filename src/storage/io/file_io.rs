use crate::configs::config::Config::DB_PATH;
use crate::enums::page_types::PageType;
use crate::utils::io_utils::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
pub struct IOEngine;

impl IOEngine {
    /// Creates a new database with the given name.
    /// This function will:
    /// - Create the base directory for the database
    /// - Create subdirectories (`tables`, `index`, `fst`)
    /// - Create a catalog file with the name `{db_name}.clog`
    pub fn create_db(db_name: &str) -> Result<(), String> {
        let base = PathBuf::from(DB_PATH).join(db_name);

        // Create base and subdirectories
        ["", "tables", "indexes", "fst"]
            .iter()
            .try_for_each(|folder| {
                let path = if folder.is_empty() {
                    base.clone()
                } else {
                    base.join(folder)
                };
                create_dir(&path).map_err(|e| e.to_string())
            })?;

        // Create the catalog file
        let clog_path = base.join(format!("{db_name}.bin"));
        create_file(&clog_path)?;
        Ok(())
    }

    /// Creates a new table inside the specified database.
    /// This function will:
    /// - Create a table file in the `tables` folder with the name `{table_name}.bin`
    /// - Create a free space tracker file in the `fst` folder with the same name
    pub fn create_table(db_name: &str, table_name: &str) -> Result<(), String> {
        let base = PathBuf::from(DB_PATH).join(db_name);
        // Create table and free space tracker file for table file.
        for folder in ["tables", "fst"] {
            let path = base.join(folder).join(format!("{table_name}.bin"));
            create_file(&path)?;
        }
        Ok(())
    }

    /// Creates an index file for the specified database.
    /// This function will:
    /// - Create the index file inside the `index` folder with the given `index_name`
    pub fn create_index(db_name: &str, index_name: &str) -> Result<(), String> {
        let base = PathBuf::from(DB_PATH).join(db_name);
        let path = base.join("indexes").join(format!("{index_name}.bin"));
        create_file(&path)?;
        Ok(())
    }

    /// Deletes a database and its contents.
    /// This function will:
    /// - Remove the entire database directory along with its files and subdirectories
    pub fn delete_db(db_name: &str) -> Result<(), String> {
        let base = PathBuf::from(DB_PATH).join(db_name);
        // Delete database directory
        remove_dir(&base)?;
        Ok(())
    }

    /// Deletes table inside the specified database.
    /// This function will:
    /// - Delete table file in the `tables` folder with the name `{table_name}.bin`
    /// - Delete free space tracker file in the `fst` folder with the same name
    pub fn delete_table(db_name: &str, table_name: &str) -> Result<(), String> {
        let base = PathBuf::from(DB_PATH).join(db_name);
        // Create table and free space tracker file for table file.
        for folder in ["tables", "fst"] {
            let path = base.join(folder).join(format!("{table_name}.bin"));
            remove_file(&path)?;
        }
        Ok(())
    }

    /// Delete an index file for the specified database.
    /// This function will:
    /// - Delete the index file inside the `index` folder with the given `index_name`
    pub fn delete_index(db_name: &str, index_name: &str) -> Result<(), String> {
        let base = PathBuf::from(DB_PATH).join(db_name);
        let path = base.join("indexes").join(format!("{index_name}.bin"));
        remove_file(&path)?;
        Ok(())
    }

    /// Builds the file path for a specific page type inside a database.
    ///
    /// # Arguments
    /// - `db_name` - Name of the database.
    /// - `file_name` - Base name of the file (without extension).
    /// - `page_type` - Type of the page (DataPage, IndexPage, etc.).
    ///
    /// # Returns
    /// - `PathBuf` to the appropriate file based on `page_type`.
    fn _create_path(db_name: &str, file_name: &str, page_type: &PageType) -> PathBuf {
        let base = PathBuf::from(DB_PATH).join(db_name);
        let file_name = format!("{file_name}.bin");
        let path = match page_type {
            PageType::DataPage => base.join("tables").join(file_name),
            PageType::IndexPage => base.join("indexes").join(file_name),
            PageType::FsmPage => base.join("fst").join(file_name),
            PageType::CatlogPage => base.join(format!("{db_name}.bin")),
        };
        path
    }

    /// Calculates the total number of pages in a file based on its size.
    ///
    /// # Arguments
    /// - `db_name` - Name of the database.
    /// - `file_name` - Target file inside the database.
    /// - `page_type` - Type of the pages inside the file.
    ///
    /// # Returns
    /// - `Ok(u32)` with number of pages or an error message.
    pub fn calculate_total_pages(
        db_name: &str,
        file_name: &str,
        page_type: PageType,
    ) -> Result<u32, String> {
        let path = Self::_create_path(db_name, file_name, &page_type);
        let len = fs::metadata(path).map_err(|e| e.to_string())?.len();
        println!("{len}");
        let no_of_pages = match page_type {
            PageType::CatlogPage => 1,
            _ => len / page_type.size_in_bytes(),
        };
        Ok(no_of_pages as u32)
    }

    /// Calculates the byte offset for reading a specific page from a file.
    ///
    /// # Arguments
    /// - `page_type` - The type of the page.
    /// - `page_no` - Page number to read.
    ///
    /// # Returns
    /// - `u64` offset in bytes from the beginning of the file.
    pub fn calculate_offsets_to_read(page_type: &PageType, page_no: u32) -> u64 {
        let mut start_offset: u64 = page_type.size_in_bytes() * page_no as u64;
        if let PageType::CatlogPage = page_type {
            start_offset = 0;
        }
        start_offset
    }

    /// Appends a new page to the end of a file.
    ///
    /// # Arguments
    /// - `db_name` - Name of the database.
    /// - `file_name` - Target file name.
    /// - `page_buffer` - Buffer containing the page data.
    /// - `page_type` - Type of page to append.
    ///
    /// # Returns
    /// - `Ok(())` if successful, or error message if fail.
    ///
    /// # Errors
    /// - Rejects appending to `CatlogPage`.
    pub fn append_page(
        db_name: &str,
        file_name: &str,
        page_buffer: &[u8],
        page_type: PageType,
    ) -> Result<(), String> {
        if let PageType::CatlogPage = page_type {
            return Err(String::from("Cannot append a page to catlog table"));
        }
        let path = Self::_create_path(db_name, file_name, &page_type);

        let mut file = OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        file.write_all(&page_buffer).map_err(|e| e.to_string())?;
        file.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Reads a page from a file into the provided buffer.
    ///
    /// # Arguments
    /// - `db_name` - Name of the database.
    /// - `file_name` - Target file name.
    /// - `buffer` - Mutable buffer to fill with read data.
    /// - `page_type` - Type of the page to read.
    /// - `page_no` - Page number to read from.
    ///
    /// # Returns
    /// - `Ok(())` on success, or error message if fail.
    pub fn read_page(
        db_name: &str,
        file_name: &str,
        buffer: &mut [u8],
        page_type: PageType,
        page_no: u32,
    ) -> Result<(), String> {
        let path = Self::_create_path(db_name, file_name, &page_type);
        let start_offset = Self::calculate_offsets_to_read(&page_type, page_no);
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        file.seek(SeekFrom::Start(start_offset))
            .map_err(|e| e.to_string())?;
        file.read_exact(buffer).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Updates a specific page in a file by overwriting it.
    ///
    /// # Arguments
    /// - `db_name` - Name of the database.
    /// - `file_name` - Target file name.
    /// - `buffer` - Buffer containing new page data.
    /// - `page_type` - Type of the page to update.
    /// - `page_no` - Page number to overwrite.
    ///
    /// # Returns
    /// - `Ok(())` on success, or error message if fail.
    pub fn update_page(
        db_name: &str,
        file_name: &str,
        buffer: &mut [u8],
        page_type: PageType,
        page_no: u32,
    ) -> Result<(), String> {
        let path = Self::_create_path(db_name, file_name, &page_type);
        let start_offset = Self::calculate_offsets_to_read(&page_type, page_no);
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        file.seek(SeekFrom::Start(start_offset))
            .map_err(|e| e.to_string())?;
        file.write_all(buffer).map_err(|e| e.to_string())?;
        Ok(())
    }
}
