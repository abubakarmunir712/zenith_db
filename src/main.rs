mod configs;
mod enums;
mod storage;
mod types;
mod utils;

use enums::datatypes::DataType;
use storage::catalog::catalog::{CatalogTable, ColumnInfo};
use storage::page::page::Page;
use storage::page::page_header::PageHeader;
use storage::page::slot::Slot;
use storage::record::record_manager::RecordManager;
use types::bool::BOOL;
use types::char::CHAR;
use types::int::INT;

fn test_record() {
    // Step 1: Create a CatalogTable with fixed and variable columns
    let catalog = CatalogTable {
        table_name: "users".to_string(),
        database_name: "main_db".to_string(),
        no_of_fixed_columns: 2,
        no_of_variable_columns: 1,
        fixed_columns: vec![
            ColumnInfo {
                column_name: "ID".to_string(),
                max_data_size: 4,
                data_type: DataType::INT(INT::new(4)),
            },
            ColumnInfo {
                column_name: "IsActive".to_string(),
                max_data_size: 1,
                data_type: DataType::BOOL(BOOL::new(true)),
            },
        ],
        variable_columns: vec![ColumnInfo {
            column_name: "Name".to_string(),
            max_data_size: 50,
            data_type: DataType::VARCHAR(CHAR::new(50, "TestName").unwrap()),
        }],
    };

    // Step 2: Create an empty Page to hold the serialized data
    let mut page = Page::new(2, 1);
    println!(
        "\n=== Test Record: Page Initialization ===\nInitial page data capacity: {}\n",
        page.data().capacity()
    );
    let mut page: [u8; 4096] = page.serialize();
    let mut page = Page::deserialize(&page);
    println!("Deserialized page data capacity: {}\n", page.data().capacity());

    // Step 3: Prepare the data that will be serialized into the page
    let record_data: Vec<DataType> = vec![
        DataType::INT(INT::new(4)),      // INT column
        DataType::BOOL(BOOL::new(true)), // BOOL column
        DataType::VARCHAR(CHAR::new(50, "TestName djhsdjskjdksjdksjdks").unwrap()), // VARCHAR column
    ];

    // Step 4: Serialize the data into the page using RecordManager
    let record_start = 0; // Starting position in the page buffer
    RecordManager::from_human_readable(&mut page, &catalog, record_start, record_data);

    // Step 5: Deserialize the data from the page
    let deserialized_data = RecordManager::to_human_readable(&page, &catalog, record_start);

    // Step 6: Print the deserialized data to verify correctness
    println!("=== Deserialized Record Data ===");
    for (i, data) in deserialized_data.iter().enumerate() {
        match data {
            DataType::INT(i) => println!("Column {}: INT    = {:<10}", i.value + 1, i.value),
            DataType::BOOL(b) => println!("Column {}: BOOL   = {:<10}", i + 1, b.value),
            DataType::VARCHAR(v) => println!("Column {}: VARCHAR = {:<10}", i + 1, v.value),
            _ => println!("Column {}: Unknown DataType", i + 1),
        }
    }
    println!();
}

fn test_page() {
    let mut slot_table: Vec<Slot> = Vec::new();
    let page_header = PageHeader::new(1, 1, 31 + 5, 3, 4056);
    let slot = Slot::new(0, 5, 1);
    slot_table.push(slot);
    let slot = Slot::new(5, 6, 0);
    slot_table.push(slot);
    let slot = Slot::new(11, 9, 1);
    slot_table.push(slot);
    let slot = Slot::new(20, 6, 0);
    slot_table.push(slot);
    let slot = Slot::new(31, 5, 0);
    slot_table.push(slot);
    
    println!("\n=== Test Page: Before Slot Deletion ===");
    let mut page = Page::new_test(page_header, slot_table);
    println!("Free Space Offset: {}", page.page_header().free_space_offset());
    
    println!("\nSlot Table:");
    println!("{:-<45}", "");
    println!("| {:<10} | {:<12} | {:<12} |", "Offset", "Record Size", "Deleted");
    println!("{:-<45}", "");
    for slot in page.slot_table() {
        println!(
            "| {:<10} | {:<12} | {:<12} |",
            slot.record_offset(),
            slot.record_size(),
            slot.is_deleted()
        );
    }
    println!("{:-<45}", "");
    
    println!("\nFree Space Table:");
    println!("{:-<30}", "");
    println!("| {:<10} | {:<10} |", "Offset", "Size");
    println!("{:-<30}", "");
    let free_space = page.free_space_table();
    for (offset, size) in &free_space {
        println!("| {:<10} | {:<10} |", offset, size);
    }
    println!("{:-<30}", "");
    
    page.delete_slot(1).unwrap();
    
    println!("\n=== Test Page: After Slot Deletion ===");
    println!("Free Space Offset: {}", page.page_header().free_space_offset());
    
    println!("\nSlot Table:");
    println!("{:-<45}", "");
    println!("| {:<10} | {:<12} | {:<12} |", "Offset", "Record Size", "Deleted");
    println!("{:-<45}", "");
    for slot in page.slot_table() {
        println!(
            "| {:<10} | {:<12} | {:<12} |",
            slot.record_offset(),
            slot.record_size(),
            slot.is_deleted()
        );
    }
    println!("{:-<45}", "");
    
    println!("\nFree Space Table:");
    println!("{:-<30}", "");
    println!("| {:<10} | {:<10} |", "Offset", "Size");
    println!("{:-<30}", "");
    let free_space = page.free_space_table();
    for (offset, size) in &free_space {
        println!("| {:<10} | {:<10} |", offset, size);
    }
    println!("{:-<30}", "");

    let max_free = page.max_free_space().unwrap().1;
    println!("\nMaximum Free Space Size: {}\n", max_free);
}

fn test_slot() {
    let mut slot_table: Vec<Slot> = Vec::new();
    let page_header = PageHeader::new(1, 1, 31 + 5, 3, 4056);
    let slot = Slot::new(0, 5, 1);
    slot_table.push(slot);
    let slot = Slot::new(5, 6, 0);
    slot_table.push(slot);
    let slot = Slot::new(11, 9, 1);
    slot_table.push(slot);
    let slot = Slot::new(20, 6, 0);
    slot_table.push(slot);
    let slot = Slot::new(31, 5, 0);
    slot_table.push(slot);

    println!("\n=== Test Slot: Before Serialization ===");
    let page = Page::new_test(page_header, slot_table);
    println!("Slot Table:");
    println!("{:-<45}", "");
    println!("| {:<10} | {:<12} | {:<12} |", "Offset", "Record Size", "Deleted");
    println!("{:-<45}", "");
    for slot in page.slot_table() {
        println!(
            "| {:<10} | {:<12} | {:<12} |",
            slot.record_offset(),
            slot.record_size(),
            slot.is_deleted()
        );
    }
    println!("{:-<45}", "");
    
    println!("\n=== Test Slot: After Deserialization ===");
    let buffer: [u8; 4096] = page.serialize();
    let page = Page::deserialize(&buffer);
    println!("Slot Table:");
    println!("{:-<45}", "");
    println!("| {:<10} | {:<12} | {:<12} |", "Offset", "Record Size", "Deleted");
    println!("{:-<45}", "");
    for slot in page.slot_table() {
        println!(
            "| {:<10} | {:<12} | {:<12} |",
            slot.record_offset(),
            slot.record_size(),
            slot.is_deleted()
        );
    }
    println!("{:-<45}", "");
    println!();
}



// These are temporary test functions for verifying Page, Record, and Slot functionality.
// They should be removed after moving tests to the dedicated test directory (e.g., tests/).
fn main() {
    println!("\n=========================================== PAGE TEST ===========================================");
    test_page();
    println!("\n----------------------------------------------\n");
    
    println!("\n========================================== RECORD TEST ==========================================");
    test_record();
    println!("\n----------------------------------------------\n");
    
    println!("\n=========================================== SLOT TEST ===========================================");
    test_slot();
    println!("\n----------------------------------------------\n");
}