use ZenithDB::configs::config::Config::{CATLOG_PAGE_SIZE, REF_PAGE_SIZE};
use ZenithDB::configs::db_internal_configs::DbConfigs::MAX_REF_SIZE;
use ZenithDB::enums::cascading_type::ForeignKeyAction;
use ZenithDB::enums::catlog_errors::CatalogError;
use ZenithDB::enums::datatypes::DataType;
use ZenithDB::enums::page_types::PageType;
use ZenithDB::storage::catalog::entries::column_entry::ColumnEntry;
use ZenithDB::storage::catalog::entries::ref_entry::{RefEntry, RefPair};
use ZenithDB::storage::catalog::entries::table_entry::TableEntry;
use ZenithDB::storage::catalog::maps::column_map::ColumnMap;
use ZenithDB::storage::catalog::maps::ref_map::RefMap;
use ZenithDB::storage::catalog::maps::table_map::TableMap;
use ZenithDB::storage::io::file_io::IOEngine;

#[test]
fn test_table_map_serialize_deserialize() {
    let mut table_map = TableMap::new();

    for i in 0..400 {
        let name = format!("table_{}", i);
        let mut entry = TableEntry::new(name.clone(), i).unwrap();
        for _ in 0..i + 1 {
            entry.increase_columns();
        }

        for _ in 0..i % 5 {
            entry.increase_no_of_cols_in_pk();
        }
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
    let mut entry1 = TableEntry::new("users".to_string(), 1).unwrap();
    entry1.increase_columns();
    entry1.increase_columns();
    entry1.increase_columns();

    entry1.increase_no_of_cols_in_pk();
    let mut entry2 = TableEntry::new("orders".to_string(), 2).unwrap();
    entry2.increase_columns();
    entry2.increase_columns();
    entry2.increase_columns();
    entry2.increase_columns();
    entry2.increase_columns();
    entry2.increase_no_of_cols_in_pk();
    entry2.increase_no_of_cols_in_pk();
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
    assert_eq!(users_entry.no_of_cols_in_primary_key(), 1);

    let orders_entry = deserialized.map.get("orders").unwrap();
    assert_eq!(orders_entry.oid(), 2);
    assert_eq!(orders_entry.columns(), 5);
    assert_eq!(orders_entry.no_of_cols_in_primary_key(), 2);

    // Step 8: Clean up
    IOEngine::delete_db(db_name).unwrap();
}

#[test]
fn test_column_map_serialize_deserialize() {
    let mut column_map = ColumnMap::new();

    for i in 0..400 {
        let name = format!("col_{}", i);
        let mut entry = ColumnEntry::new(name.clone(), i, DataType::CHAR, 10).unwrap();
        if i % 2 == 0 {
            entry.make_nullable();
        }
        column_map.create_column(entry).unwrap();
    }

    let mut buffer = [0u8; CATLOG_PAGE_SIZE as usize];
    column_map.serialize(&mut buffer);

    let deserialized = ColumnMap::deserialize(&buffer);

    assert_eq!(deserialized.no_of_columns(), 400);

    for i in 0..400 {
        let name = format!("col_{}", i);
        let entry = deserialized.map().get(&name).unwrap();
        assert_eq!(entry.oid(), i);
        // assert_eq!( DataType::CHAR, entry.datatype);
        assert_eq!(entry.max_size(), 10);
        assert_eq!(entry.is_nullable(), i % 2 == 0);
    }
}

#[test]
fn test_column_map_io() {
    let db_name = "test_db_col_io";

    // Step 1: Create DB
    IOEngine::create_db(db_name).unwrap();

    // Step 2: Create ColumnMap and insert entries
    let mut column_map = ColumnMap::new();
    let col1 = ColumnEntry::new("id".to_string(), 1, DataType::INT, 4).unwrap();
    let mut col2 = ColumnEntry::new("name".to_string(), 2, DataType::VARCHAR, 50).unwrap();
    col2.make_nullable();

    column_map.create_column(col1).unwrap();
    column_map.create_column(col2).unwrap();

    // Step 3: Serialize ColumnMap to buffer
    let mut write_buffer = [0u8; CATLOG_PAGE_SIZE as usize];
    column_map.serialize(&mut write_buffer);

    // Step 4: Append catalog page with serialized buffer
    IOEngine::append_page(db_name, db_name, &write_buffer, PageType::RefPage).unwrap();

    // Step 5: Read back page
    let mut read_buffer = [0u8; CATLOG_PAGE_SIZE as usize];
    IOEngine::read_page(db_name, db_name, &mut read_buffer, PageType::RefPage, 0).unwrap();

    // Step 6: Deserialize ColumnMap from read buffer
    let deserialized = ColumnMap::deserialize(&read_buffer);

    // Step 7: Assertions
    assert_eq!(deserialized.no_of_columns(), 2);

    // Check first column (id)
    let id = deserialized.map().get("id").unwrap();
    assert_eq!(id.oid(), 1);
    // assert_eq!(id.datatype, DataType::INT);
    assert_eq!(id.max_size(), 4);
    assert!(!id.is_nullable());

    // Check second column (name)
    let name = deserialized.map().get("name").unwrap();
    assert_eq!(name.oid(), 2);
    // assert_eq!(name.datatype(), DataType::VARCHAR);
    assert_eq!(name.max_size(), 50);
    assert!(name.is_nullable());

    // Step 8: Clean up
    // IOEngine::delete_db(db_name).unwrap();
}

#[test]
fn test_insert_and_serialize_deserialize_ref_map() {
    let mut ref_map = RefMap::new();

    for i in 0..5 {
        let i = i as u16;
        let ref_entry = RefEntry::new(
            vec![RefPair::new(Some(i), Some(i + 1), Some(i + 2), Some(i + 3))],
            ForeignKeyAction::Cascade,
        )
        .unwrap();

        ref_map.insert_ref(ref_entry).unwrap();
    }

    let mut buffer = [0u8; REF_PAGE_SIZE as usize];
    ref_map.serialize(&mut buffer).unwrap();

    let deserialized = RefMap::deserialize(&buffer).unwrap();

    assert_eq!(deserialized.no_of_ref(), 5);
    let entry = &deserialized.map()[0];
    let pairs = entry.references();
    assert_eq!(pairs.len(), 1);
    let pair = &pairs[0];
    assert_eq!(pair.f_table_oid(), None);
    assert_eq!(pair.f_column_oid(), Some(1));
    assert_eq!(pair.r_table_oid(), Some(2));
    assert_eq!(pair.r_column_oid(), Some(3));
    assert_eq!(
        entry.cascading_type().to_oid(),
        ForeignKeyAction::Cascade.to_oid()
    );


    let entry = &deserialized.map()[1];
    let pairs = entry.references();
    assert_eq!(pairs.len(), 1);
    let pair = &pairs[0];
    assert_eq!(pair.f_table_oid(), Some(1));
    assert_eq!(pair.f_column_oid(), Some(2));
    assert_eq!(pair.r_table_oid(), Some(3));
    assert_eq!(pair.r_column_oid(), Some(4));
    assert_eq!(
        entry.cascading_type().to_oid(),
        ForeignKeyAction::Cascade.to_oid()
    );
}

#[test]
fn test_insert_ref_exceeds_max_limit() {
    let mut ref_map = RefMap::new();

    for _ in 0..MAX_REF_SIZE {
        let entry = RefEntry::new(vec![], ForeignKeyAction::Cascade).unwrap();
        ref_map.insert_ref(entry).unwrap();
    }

    let too_much = RefEntry::new(vec![], ForeignKeyAction::Cascade).unwrap();
    let result = ref_map.insert_ref(too_much);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        CatalogError::SysMaxRefPerPageLimitExceeded.message()
    );
}
