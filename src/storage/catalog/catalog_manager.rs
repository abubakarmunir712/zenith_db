use std::sync::{Arc, RwLock};

use crate::{
    configs::db_internal_configs::DbConfigs::{MAX_COLUMNS_LIMIT, MAX_TABLES_LIMIT},
    enums::types::{
        catalog_types::{CatalogData, CatalogType},
        datatypes::DataType,
    },
    oid::oid_manager,
    storage::buffer::catalog_buffer::CatalogBuffer,
};

use super::{
    entries::{column_entry::ColumnEntry, table_entry::TableEntry},
    maps::{column_map::ColumnMap, table_map::TableMap},
};

pub struct CatalogManager {
    pub catlog_buffer: Arc<CatalogBuffer>,
}
impl CatalogManager {
    // Needs to be moved outside
    /// Fetches the page containing the TableMap for the given DB.
    ///
    /// `mark_dirty` indicates if the page should be marked dirty (modified).
    /// Page number for TableMap is always 0.
    pub fn get_table_map(
        &self,
        db_name: &str,
        mark_dirty: bool,
    ) -> Result<Arc<RwLock<CatalogData>>, String> {
        let page: Arc<RwLock<CatalogData>> =
            self.catlog_buffer
                .get_page(db_name, CatalogType::TableMap, 0, mark_dirty)?;
        Ok(page)
    }

    // Needs to be moved outside
    /// Fetches the ColumnMap page for the given table.
    ///
    /// The page number is determined by the table's OID.
    pub fn get_column_map(
        &self,
        db_name: &str,
        mark_dirty: bool,
        table_entry: &TableEntry,
    ) -> Result<Arc<RwLock<CatalogData>>, String> {
        let page_number = table_entry.oid();
        let page = self.catlog_buffer.get_page(
            db_name,
            CatalogType::ColumnMap,
            page_number as u32,
            mark_dirty,
        )?;
        Ok(page)
    }

    /// Creates a new table entry in the table map.
    ///
    /// - Generates a new OID using the bitmap.
    /// - Inserts the table into the map.
    /// - Deletes the OID from the bitmap afterward.
    pub fn create_table(table_name: &str, table_map: &mut TableMap) -> Result<u16, String> {
        let bitmask: &[u8; MAX_TABLES_LIMIT / 8] = table_map.table_oid_bitmap();
        let oid = oid_manager::generate_oid(bitmask).unwrap() as u16;
        let table_entry = TableEntry::new(table_name.to_string(), oid)?;
        table_map.create_table(table_entry);
        let bitmask: &mut [u8; MAX_TABLES_LIMIT / 8] = table_map.table_oid_bitmap_mut();
        oid_manager::delete_oid(bitmask, oid);
        Ok(oid)
    }

    /// Creates a new column in the column map.
    ///
    /// - Supports fixed and variable-sized types (uses `max_size`).
    ///
    /// # Params:
    /// - `column_name`: name of the column
    /// - `datatype`: enum like Int, Varchar, etc.
    /// - `max_size`: optional, required for types like Varchar/Char
    pub fn create_column(
        &self,
        column_name: &str,
        table_entry: &mut TableEntry,
        column_map: &mut ColumnMap,
        datatype: DataType,
        mut max_size: Option<u32>,
    ) -> Result<u16, String> {
        let bitmask: &[u8; MAX_COLUMNS_LIMIT / 8] = column_map.column_oid_bitmap();
        let oid = oid_manager::generate_oid(bitmask).unwrap() as u16;
        if max_size.is_none() {
            max_size = Some(datatype.size() as u32)
        }
        let column_entry =
            ColumnEntry::new(column_name.to_string(), oid, datatype, max_size.unwrap())?;
        column_map.create_column(column_entry);
        table_entry.increase_columns();
        let bitmask = column_map.column_oid_bitmap_mut();
        oid_manager::delete_oid(bitmask, oid);
        Ok(oid)
    }

    pub fn delete_table(table_name: &str, table_map: &mut TableMap) -> u16 {
        let oid = table_map.delete_table(table_name);
        let bitmask = table_map.table_oid_bitmap_mut();
        oid_manager::undelete_oid(bitmask, oid);
        oid
    }

    pub fn delete_column(
        column_name: &str,
        table_entry: &mut TableEntry,
        column_map: &mut ColumnMap,
    ) -> u16 {
        let oid = column_map.delete_column(column_name);
        table_entry.decrease_columns();
        let bitmask = column_map.column_oid_bitmap_mut();
        oid_manager::delete_oid(bitmask, oid);
        oid
    }
}
