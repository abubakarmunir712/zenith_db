use super::super::entries::table_entry::TableEntry;
use crate::{
    configs::{config::Config::CATLOG_PAGE_SIZE, db_internal_configs::DbConfigs::MAX_TABLES_LIMIT},
    enums::catalog_errors::CatalogError,
};
use std::collections::HashMap;

pub struct TableMap {
    no_of_tables: u16,
    table_oid_bitmap: [u8; MAX_TABLES_LIMIT / 8],
    map: HashMap<String, TableEntry>,
}

impl TableMap {
    pub fn new() -> Self {
        let table_oid_bitmap = [0; MAX_TABLES_LIMIT / 8];
        Self {
            no_of_tables: 0,
            table_oid_bitmap,
            map: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, table_entry: TableEntry) {
        self.map
            .insert(table_entry.table_name().to_string(), table_entry);
        self.no_of_tables += 1;
    }

    pub fn delete_table(&mut self, table_name: &str)->u16 {
        let entry = self.map.remove(table_name).unwrap();
        self.no_of_tables -= 1;
        entry.oid()
    }

    pub fn get_table(&self, table_name: &str) -> Option<&TableEntry> {
        self.map().get(table_name)
    }

    pub fn get_table_as_mut(&mut self, table_name: &str) -> Option<&mut TableEntry> {
        self.map.get_mut(table_name)
    }

    pub fn serialize(&self, buffer: &mut [u8]) {
        // Serialize number of tables
        buffer[0..2].copy_from_slice(&self.no_of_tables.to_le_bytes());

        // Serialize table_oid_bitmap
        let bitmap_size = MAX_TABLES_LIMIT / 8;
        let bitmap_end = 2 + bitmap_size;
        buffer[2..bitmap_end].copy_from_slice(&self.table_oid_bitmap);

        // Serialize each TableEntry
        let mut offset = bitmap_end;
        for entry in self.map.values() {
            let serialized = entry.serialize();
            buffer[offset..offset + serialized.len()].copy_from_slice(&serialized);
            offset += serialized.len();
        }
    }

    pub fn deserialize(buffer: &[u8]) -> Self {
        let no_of_tables = u16::from_le_bytes([buffer[0], buffer[1]]);

        let bitmap_size = MAX_TABLES_LIMIT / 8;
        let bitmap_end = 2 + bitmap_size;

        let mut table_oid_bitmap = [0u8; MAX_TABLES_LIMIT / 8];
        table_oid_bitmap.copy_from_slice(&buffer[2..bitmap_end]);

        let mut offset = bitmap_end;
        let mut map = HashMap::new();

        for _ in 0..no_of_tables {
            let entry = TableEntry::deserialize(&buffer[offset..]);
            offset += 70;
            map.insert(entry.table_name().to_string(), entry);
        }

        Self {
            no_of_tables,
            table_oid_bitmap,
            map,
        }
    }

    // Immutable getter for number of tables
    pub fn no_of_tables(&self) -> u16 {
        self.no_of_tables
    }

    // Immutable getter for the table OID bitmap
    pub fn table_oid_bitmap(&self) -> &[u8; MAX_TABLES_LIMIT / 8] {
        &self.table_oid_bitmap
    }

    // Mutable getter for the table OID bitmap
    pub fn table_oid_bitmap_mut(&mut self) -> &mut [u8; MAX_TABLES_LIMIT / 8] {
        &mut self.table_oid_bitmap
    }

    // Immutable getter for the table map
    pub fn map(&self) -> &HashMap<String, TableEntry> {
        &self.map
    }

    // Mutable getter for the table map
    pub fn map_mut(&mut self) -> &mut HashMap<String, TableEntry> {
        &mut self.map
    }
}
