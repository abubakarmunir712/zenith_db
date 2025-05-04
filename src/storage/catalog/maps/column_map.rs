use super::super::entries::column_entry::ColumnEntry;
use crate::configs::db_internal_configs::DbConfigs::MAX_COLUMNS_LIMIT;
use std::collections::HashMap;

pub struct ColumnMap {
    no_of_columns: u16,
    column_oid_bitmap: [u8; MAX_COLUMNS_LIMIT / 8],
    map: HashMap<String, ColumnEntry>,
}
