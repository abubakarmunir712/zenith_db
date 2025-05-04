use super::super::entries::table_entry::TableEntry;
use crate::configs::db_internal_configs::DbConfigs::MAX_TABLES_LIMIT;
use std::collections::HashMap;

pub struct TableMap {
    no_of_tables: u16,
    table_oid_bitmap: [u8; MAX_TABLES_LIMIT / 8],
    map: HashMap<String, TableEntry>,
}
