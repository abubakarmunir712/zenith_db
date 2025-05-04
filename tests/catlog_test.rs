use ZenithDB::configs::config::Config::CATLOG_PAGE_SIZE;
use ZenithDB::storage::catalog::maps::table_map::TableMap;
use ZenithDB::storage::catalog::entries::table_entry::TableEntry;

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
