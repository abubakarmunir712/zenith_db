use super::super::entries::column_entry::ColumnEntry;
use crate::{
    configs::db_internal_configs::DbConfigs::MAX_COLUMNS_LIMIT, enums::catlog_errors::CatalogError,
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
}
