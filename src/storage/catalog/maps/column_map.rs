use super::super::entries::column_entry::ColumnEntry;
use crate::configs::{
    config::Config::CATLOG_PAGE_SIZE, db_internal_configs::DbConfigs::MAX_COLUMNS_LIMIT,
};
use std::collections::HashMap;

pub struct ColumnMap {
    no_of_columns: u16,
    column_oid_bitmap: [u8; MAX_COLUMNS_LIMIT / 8],
    map: HashMap<String, ColumnEntry>,
}

impl ColumnMap {
    pub fn new() -> Self {
        let column_oid_bitmap = [0; MAX_COLUMNS_LIMIT / 8];
        Self {
            no_of_columns: 0,
            column_oid_bitmap,
            map: HashMap::new(),
        }
    }

    pub fn create_column(&mut self, column_entry: ColumnEntry) -> Result<(), &str> {
        self.map
            .insert(column_entry.column_name.clone(), column_entry);
        self.no_of_columns += 1;
        Ok(())
    }

    pub fn serialize(&self, buffer: &mut [u8; CATLOG_PAGE_SIZE as usize]) {
        // Write number of columns
        buffer[0..2].copy_from_slice(&self.no_of_columns.to_le_bytes());

        // Write column_oid_bitmap
        let bitmap_size = MAX_COLUMNS_LIMIT / 8;
        let bitmap_end = 2 + bitmap_size;
        buffer[2..bitmap_end].copy_from_slice(&self.column_oid_bitmap);

        // Write each ColumnEntry
        let mut offset = bitmap_end;
        for entry in self.map.values() {
            let serialized = entry.serialize();
            buffer[offset..offset + serialized.len()].copy_from_slice(&serialized);
            offset += serialized.len();
        }
    }

    pub fn deserialize(buffer: &[u8; CATLOG_PAGE_SIZE as usize]) -> Self {
        let no_of_columns = u16::from_le_bytes([buffer[0], buffer[1]]);

        let bitmap_size = MAX_COLUMNS_LIMIT / 8;
        let bitmap_end = 2 + bitmap_size;

        let mut column_oid_bitmap = [0u8; MAX_COLUMNS_LIMIT / 8];
        column_oid_bitmap.copy_from_slice(&buffer[2..bitmap_end]);

        let mut offset = bitmap_end;
        let mut map = HashMap::new();

        for _ in 0..no_of_columns {
            let entry = ColumnEntry::deserialize(&buffer[offset..offset + 76]);
            offset += 76;
            map.insert(entry.column_name.clone(), entry);
        }

        Self {
            no_of_columns,
            column_oid_bitmap,
            map,
        }
    }

    // Getters
    pub fn no_of_columns(&self) -> u16 {
        self.no_of_columns
    }

    pub fn column_oid_bitmap(&self) -> &[u8; MAX_COLUMNS_LIMIT / 8] {
        &self.column_oid_bitmap
    }

    pub fn map(&self) -> &HashMap<String, ColumnEntry> {
        &self.map
    }
}
