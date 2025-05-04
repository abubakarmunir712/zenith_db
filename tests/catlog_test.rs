use ZenithDB::configs::config::Config::CATLOG_PAGE_SIZE;
use ZenithDB::enums::page_types::PageType;
use ZenithDB::storage::catalog::entries::table_entry::TableEntry;
use ZenithDB::storage::catalog::maps::table_map::TableMap;
use ZenithDB::storage::io::file_io::IOEngine;

#[test]
fn test_table_map_serialize_deserialize() {
    let mut table_map = TableMap::new();

    for i in 0..400 {
        let name = format!("table_{}", i);
        let entry = TableEntry::new(name.clone(), i, i + 1, i as u32 * 10, (i % 5) as u8).unwrap();
        table_map.create_table(entry).unwrap();
    }

    let mut buffer = [0u8; CATLOG_PAGE_SIZE as usize];
    table_map.serialize(&mut buffer);

    let deserialized = TableMap::deserialize(&buffer);

    assert_eq!(deserialized.no_of_tables, 400);

    for i in 0..400 {
        let name = format!("table_{}", i);
        let entry = deserialized.map.get(&name).unwrap();
        assert_eq!(entry.oid(), i);
        assert_eq!(entry.columns(), i + 1);
        assert_eq!(entry.col_map_pg_num(), i as u32 * 10);
        assert_eq!(entry.no_of_cols_in_primary_key(), (i % 5) as u8);
    }
}

#[test]
fn test_table_map_io() {
    let db_name = "test_db_table_io";

    // Step 1: Create DB
    IOEngine::create_db(db_name).unwrap();

    // Step 2: Create TableMap and insert entries
    let mut table_map = TableMap::new();
    let entry1 = TableEntry::new("users".to_string(), 1, 3, 10, 1).unwrap();
    let entry2 = TableEntry::new("orders".to_string(), 2, 5, 11, 2).unwrap();
    table_map.create_table(entry1).unwrap();
    table_map.create_table(entry2).unwrap();

    // Step 3: Serialize TableMap to buffer
    let mut write_buffer = [0u8; CATLOG_PAGE_SIZE as usize];
    table_map.serialize(&mut write_buffer);

    // Step 4: Append catalog page with serialized buffer
    IOEngine::append_page(db_name, db_name, &write_buffer, PageType::CatlogPage).unwrap();

    // Step 5: Read back page
    let mut read_buffer = [0u8; CATLOG_PAGE_SIZE as usize];
    IOEngine::read_page(db_name, db_name, &mut read_buffer, PageType::CatlogPage, 0).unwrap();

    // Step 6: Deserialize TableMap from read buffer
    let deserialized = TableMap::deserialize(&read_buffer);

    // Step 7: Assertions
    assert_eq!(deserialized.no_of_tables, 2);
    let users_entry = deserialized.map.get("users").unwrap();
    assert_eq!(users_entry.oid(), 1);
    assert_eq!(users_entry.columns(), 3);
    assert_eq!(users_entry.col_map_pg_num(), 10);
    assert_eq!(users_entry.no_of_cols_in_primary_key(), 1);

    let orders_entry = deserialized.map.get("orders").unwrap();
    assert_eq!(orders_entry.oid(), 2);
    assert_eq!(orders_entry.columns(), 5);
    assert_eq!(orders_entry.col_map_pg_num(), 11);
    assert_eq!(orders_entry.no_of_cols_in_primary_key(), 2);

    // Step 8: Clean up
    IOEngine::delete_db(db_name).unwrap();
}
