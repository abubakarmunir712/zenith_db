use ZenithDB::{enums::types::datatypes::DataType, storage::{
    buffer::catalog_buffer::CatalogBuffer,
    catalog::{catalog_manager::CatalogManager, entries::table_entry::TableEntry, maps::column_map::ColumnMap},
    record::record::Record,
}};

#[test]
fn test_record_serialization_roundtrip() {
    // Buffers Initialization
    let c_buff = CatalogBuffer::new();

    // Manager Initialization
    let c_mngr = CatalogManager {
        catlog_buffer: c_buff,
    };
    let mut t_entry = TableEntry::new("Table1".to_string(), 1).unwrap();
    let mut c_map = ColumnMap::new();
    

    // Column Definitions
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

    // Input values
    let input_vals: Vec<String> = vec![
        "".to_string(),
        "1234567890127891".to_string(),
        "Shoaibsdjksdj".to_string(),
        "12:10:30".to_string(),
    ];

    // Create record and serialize
    let original_record = Record::new(input_vals.clone(), &c_map).unwrap();
    let bytes = original_record.serialize();

    // Deserialize back
    let deserialized_record = Record::deserialize(&bytes, &c_map);

    // Check each value matches after roundtrip
    for (original, deserialized) in original_record
        .columns()
        .iter()
        .zip(deserialized_record.columns())
    {
        assert_eq!(
            original.to_string(),
            deserialized.to_string(),
            "Mismatch in column value"
        );
    }
}
