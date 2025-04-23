use crate::configs::config::Config::DB_PATH;
use crate::utils::io_utils::*;
use std::fmt::format;
use std::path::PathBuf;
pub struct IOEngine;

impl IOEngine {
    /// Creates a new database with the given name.
    /// This function will:
    /// - Create the base directory for the database
    /// - Create subdirectories (`tables`, `index`, `fst`)
    /// - Create a catalog file with the name `{db_name}.clog`
    pub fn create_db(db_name: &str) {
        let base = PathBuf::from(DB_PATH).join(db_name);

        // Create base and subdirectories
        ["", "tables", "index", "fst"].iter().for_each(|folder| {
            let path = if folder.is_empty() {
                &base
            } else {
                &base.join(folder)
            };
            create_dir(path);
        });

        // Create the catlog file
        let clog_path = base.join(format!("{db_name}.clog"));
        create_file(&clog_path);
    }

    /// Creates a new table inside the specified database.
    /// This function will:
    /// - Create a table file in the `tables` folder with the name `{table_name}.bin`
    /// - Create a free space tracker file in the `fst` folder with the same name
    pub fn create_table(db_name: &str, table_name: &str) {
        let base = PathBuf::from(DB_PATH).join(db_name);
        // Create table and free space tracker file for table file.
        for folder in ["tables", "fst"] {
            let path = base.join(folder).join(format!("{table_name}.bin"));
            create_file(&path);
        }
    }

    /// Creates an index file for the specified database.
    /// This function will:
    /// - Create the index file inside the `index` folder with the given `index_name`
    pub fn create_index(db_name: &str, index_name: &str) {
        let base = PathBuf::from(DB_PATH).join(db_name);
        let path = base.join("index").join(format!("{index_name}.bin"));
        create_file(&path);
    }

    /// Deletes a database and its contents.
    /// This function will:
    /// - Remove the entire database directory along with its files and subdirectories
    pub fn delete_db(db_name: &str) {
        let base = PathBuf::from(DB_PATH).join(db_name);
        // Delete database directory
        remove_dir(&base);
    }

    /// Deletes table inside the specified database.
    /// This function will:
    /// - Delete table file in the `tables` folder with the name `{table_name}.bin`
    /// - Delete free space tracker file in the `fst` folder with the same name
    pub fn delete_table(db_name: &str, table_name: &str) {
        let base = PathBuf::from(DB_PATH).join(db_name);
        // Create table and free space tracker file for table file.
        for folder in ["tables", "fst"] {
            let path = base.join(folder).join(format!("{table_name}.bin"));
            remove_file(&path);
        }
    }

    /// Delete an index file for the specified database.
    /// This function will:
    /// - Delete the index file inside the `index` folder with the given `index_name`
    pub fn delete_index(db_name: &str, index_name: &str) {
        let base = PathBuf::from(DB_PATH).join(db_name);
        let path = base.join("index").join(format!("{index_name}.bin"));
        remove_file(&path);
    }
}
