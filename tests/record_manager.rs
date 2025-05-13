use ZenithDB::{
    enums::types::datatypes::DataType,
    storage::{
        buffer::catalog_buffer::CatalogBuffer,
        catalog::{
            catalog_manager::CatalogManager, entries::table_entry::TableEntry,
            maps::column_map::ColumnMap,
        },
        page::page::Page,
        record::{record::Record, record_manager::RecordManager},
    },
};

#[test]
fn test_record_insertion_and_reading() {
    // Buffers & Manager Setup
    let c_buff = CatalogBuffer::new();
    let c_mngr = CatalogManager {
        catlog_buffer: c_buff,
    };

    let mut t_entry = TableEntry::new("Table1".to_string(), 1).unwrap();
    let mut c_map = ColumnMap::new();

    // Define columns

    c_mngr
        .create_column("col1", &mut t_entry, &mut c_map, DataType::INT, None)
        .unwrap();
    c_mngr
        .create_column("col2", &mut t_entry, &mut c_map, DataType::CHAR, Some(20))
        .unwrap();
    c_mngr
        .create_column(
            "col3",
            &mut t_entry,
            &mut c_map,
            DataType::VARCHAR,
            Some(70),
        )
        .unwrap();
    c_mngr
        .create_column("col4", &mut t_entry, &mut c_map, DataType::TIME, None)
        .unwrap();

    // Make col1 nullable
    c_map.get_column_as_mut("col1").unwrap().make_nullable();

    let mut page = Page::new(0, 1);

    // Insert 3 records
    let inputs = vec![
        vec!["", "Char data 1", "Test Data 1", "10:10:10"],
        vec!["", "Char data 2", "Test Data 2", "20:10:30"],
        vec!["", "Char data 3", "Test Data 3", "17:10:30"],
    ];

    for input in inputs {
        let cols: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        let record: Vec<u8> = Record::new(cols, &c_map).unwrap().serialize();
        RecordManager::insert_record(&record, &mut page);
    }

    // Read back records
    let records: Vec<Record> = RecordManager::read_records(&page, &c_map);

    assert_eq!(records.len(), 3);

    assert_eq!(records[0].columns()[2].to_string(), "Test Data 1");
    assert_eq!(records[1].columns()[2].to_string(), "Test Data 2");
    assert_eq!(records[2].columns()[2].to_string(), "Test Data 3");
}

#[test]
fn test_record_by_value(){
    let c_buff = CatalogBuffer::new();
    let c_mngr = CatalogManager {
        catlog_buffer: c_buff,
    };

    let mut t_entry = TableEntry::new("Table1".to_string(), 1).unwrap();
    let mut c_map = ColumnMap::new();

    // Define columns

    c_mngr
        .create_column("col1", &mut t_entry, &mut c_map, DataType::INT, None)
        .unwrap();
    c_mngr
        .create_column("col2", &mut t_entry, &mut c_map, DataType::CHAR, Some(20))
        .unwrap();
    c_mngr
        .create_column(
            "col3",
            &mut t_entry,
            &mut c_map,
            DataType::VARCHAR,
            Some(70),
        )
        .unwrap();
    c_mngr
        .create_column("col4", &mut t_entry, &mut c_map, DataType::TIME, None)
        .unwrap();

    // Make col1 nullable
    c_map.get_column_as_mut("col1").unwrap().make_nullable();

    let mut page = Page::new(0, 1);

    // Insert 3 records
    let inputs = vec![
        vec!["", "Char data 2", "Test Data 1", "10:10:10"],
        vec!["", "Char data 10", "Test Data 2", "20:10:30"],
        vec!["", "Char data 2", "Test Data 3", "17:10:30"],
    ];

    for input in inputs {
        let cols: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        let record: Vec<u8> = Record::new(cols, &c_map).unwrap().serialize();
        RecordManager::insert_record(&record, &mut page);
    }

    // Read back records
    // let records: Vec<Record> = RecordManager::read_records(&page, &c_map);
    let records = RecordManager::get_records_by_value(&page, "Char data 2", "col2", &c_map);

    assert_eq!(records.len(),2);
    assert_eq!(records[0].columns()[2].to_string(),"Test Data 1");
    assert_eq!(records[1].columns()[2].to_string(),"Test Data 3");
}