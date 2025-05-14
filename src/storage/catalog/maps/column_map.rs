use super::super::entries::column_entry::ColumnEntry;
use crate::configs::{
    config::Config::CATLOG_PAGE_SIZE, db_internal_configs::DbConfigs::MAX_COLUMNS_LIMIT,
};
use std::collections::HashMap;

pub struct ColumnMap {
    no_of_columns: u16,
    column_oid_bitmap: [u8; MAX_COLUMNS_LIMIT / 8],
    map: HashMap<String, ColumnEntry>,
    ord_map: Vec<String>,
}

impl ColumnMap {
    pub fn new() -> Self {
        let column_oid_bitmap = [0; MAX_COLUMNS_LIMIT / 8];
        Self {
            no_of_columns: 0,
            column_oid_bitmap,
            map: HashMap::new(),
            ord_map: Vec::new(),
        }
    }

    pub fn create_column(&mut self, column_entry: ColumnEntry) {
        let col_name = column_entry.column_name().to_string();
        self.map.insert(col_name.to_string(), column_entry);
        self.ord_map.push(col_name.to_string());
        self.no_of_columns += 1;
    }

    pub fn delete_column(&mut self, column_name: &str) -> u16 {
        let col = self.map.remove(column_name).unwrap();
        self.no_of_columns -= 1;
        col.oid()
    }

    pub fn serialize(&self, buffer: &mut [u8]) {
        // Write number of columns
        buffer[0..2].copy_from_slice(&self.no_of_columns.to_le_bytes());

        // Write column_oid_bitmap
        let bitmap_size = MAX_COLUMNS_LIMIT / 8;
        let bitmap_end = 2 + bitmap_size;
        buffer[2..bitmap_end].copy_from_slice(&self.column_oid_bitmap);

        // Write each ColumnEntry
        let mut offset = bitmap_end;
        for col_name in &self.ord_map {
            let entry = self.map.get(col_name).unwrap();
            let serialized = entry.serialize();
            buffer[offset..offset + serialized.len()].copy_from_slice(&serialized);
            offset += serialized.len();
        }
    }

    pub fn deserialize(buffer: &[u8]) -> Self {
        let no_of_columns = u16::from_le_bytes([buffer[0], buffer[1]]);

        let bitmap_size = MAX_COLUMNS_LIMIT / 8;
        let bitmap_end = 2 + bitmap_size;

        let mut column_oid_bitmap = [0u8; MAX_COLUMNS_LIMIT / 8];
        column_oid_bitmap.copy_from_slice(&buffer[2..bitmap_end]);

        let mut offset = bitmap_end;
        let mut map = HashMap::new();
        let mut map_ord: Vec<String> = Vec::new();

        for _ in 0..no_of_columns {
            let entry = ColumnEntry::deserialize(&buffer[offset..offset + 76]);
            let col_name = entry.column_name().to_string();
            offset += 76;
            map.insert(col_name.clone(), entry);
            map_ord.push(col_name.to_string());
        }

        Self {
            no_of_columns,
            column_oid_bitmap,
            map,
            ord_map: map_ord,
        }
    }

    // Getters
    pub fn get_column(&self, column_name: &str) -> Option<&ColumnEntry> {
        self.map.get(column_name)
    }

    pub fn get_column_as_mut(&mut self, column_name: &str) -> Option<&mut ColumnEntry> {
        self.map.get_mut(column_name)
    }

    pub fn no_of_columns(&self) -> u16 {
        self.no_of_columns
    }

    pub fn column_oid_bitmap(&self) -> &[u8; MAX_COLUMNS_LIMIT / 8] {
        &self.column_oid_bitmap
    }

    pub fn column_oid_bitmap_mut(&mut self) -> &mut [u8; MAX_COLUMNS_LIMIT / 8] {
        &mut self.column_oid_bitmap
    }

    pub fn map(&self) -> &HashMap<String, ColumnEntry> {
        &self.map
    }

    pub fn ord_map(&self) -> &Vec<String> {
        &self.ord_map
    }
}
                                    