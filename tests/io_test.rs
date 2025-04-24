use ZenithDB::configs::config::Config::DB_PATH;
use ZenithDB::storage::io::file_io::IOEngine;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use ZenithDB::enums::page_types::PageType;

#[test]
fn test_db_operations() {
    let db_name = "test_db";
    let table_name = "test_table";
    let index_name = "test_index";
    let base_path = PathBuf::from(DB_PATH).join(db_name);

    // Helper function to check if a path exists
    fn path_exists(path: &Path) -> bool {
        path.exists()
    }

    // Ensure the database doesn't exist before testing
    if path_exists(&base_path) {
        remove_dir_all(&base_path).unwrap();
    }

    // Step 1: Create the database
    IOEngine::create_db(db_name).unwrap();

    // Check if the base directory exists
    assert!(path_exists(&base_path));
    // Check if the subdirectories exist
    assert!(path_exists(&base_path.join("tables")));
    assert!(path_exists(&base_path.join("indexes")));
    assert!(path_exists(&base_path.join("fst")));
    // Check if the catalog file exists
    assert!(path_exists(&base_path.join(format!("{db_name}.bin"))));

    // Step 2: Create the table
    let table_path = base_path.join("tables").join(format!("{table_name}.bin"));
    let fst_path = base_path.join("fst").join(format!("{table_name}.bin"));

    // Create the table
    IOEngine::create_table(db_name, table_name).unwrap();

    // Check if the table file and free space tracker file are created
    assert!(path_exists(&table_path));
    assert!(path_exists(&fst_path));

    // Step 3: Create the index
    let index_path = base_path.join("indexes").join(format!("{index_name}.bin"));

    // Create the index
    IOEngine::create_index(db_name, index_name).unwrap();

    // Check if the index file is created
    assert!(path_exists(&index_path));

    // Step 4: Delete the table
    IOEngine::delete_table(db_name, table_name).unwrap();

    // Check if the table file and free space tracker file are deleted
    assert!(!path_exists(&table_path));
    assert!(!path_exists(&fst_path));

    // Step 5: Delete the index
    IOEngine::delete_index(db_name, index_name).unwrap();

    // Check if the index file is deleted
    assert!(!path_exists(&index_path));

    // Step 6: Delete the database
    IOEngine::delete_db(db_name).unwrap();

    // Check if the database directory is removed
    assert!(!path_exists(&base_path));
}

#[test]
fn test_page_operations_with_catlog_page() {

    // Setup DB
    IOEngine::create_db("zdb").unwrap();
    IOEngine::create_table("zdb", "test_table").unwrap();
    IOEngine::create_index("zdb", "idx1").unwrap();

    // Append two DataPages
    let page = &[0u8; 4096];
    IOEngine::append_page("zdb", "test_table", page, PageType::DataPage).unwrap();

    let page2 = &[1u8; 4096];
    IOEngine::append_page("zdb", "test_table", page2, PageType::DataPage).unwrap();

    // Update DataPage 0
    let mut update_buf = [2u8; 4096];
    IOEngine::update_page("zdb", "test_table", &mut update_buf, PageType::DataPage, 0).unwrap();

    // Read DataPage 0
    let mut read_buf = [0u8; 4096];
    IOEngine::read_page("zdb", "test_table", &mut read_buf, PageType::DataPage, 0).unwrap();
    assert_eq!(read_buf, update_buf);

    // Handle CatlogPage: write and read
    let catlog_data = b"This is some catlog content".to_vec();
    IOEngine::update_page("zdb", "_", &mut catlog_data.clone(), PageType::CatlogPage, 0).unwrap();

    let mut catlog_read_buf = vec![0u8; catlog_data.len()];
    IOEngine::read_page("zdb", "_", &mut catlog_read_buf, PageType::CatlogPage, 0).unwrap();
    assert_eq!(catlog_read_buf, catlog_data);

    // Cleanup
    IOEngine::delete_db("zdb").unwrap();
}
