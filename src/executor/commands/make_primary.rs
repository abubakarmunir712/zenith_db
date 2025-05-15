use std::sync::Arc;

use crate::{
    enums::{
        errors::db_status::DatabaseStatus,
        types::catalog_types::{CatalogData, CatalogType},
    },
    storage::{buffer::catalog_buffer::CatalogBuffer, catalog::catalog_manager::CatalogManager},
};

pub fn make_primary(
    db_name: &str,
    column: &str,
    table: &str,
    c_manager: &CatalogManager,
    c_buffer: &Arc<CatalogBuffer>,
) -> Result<(), String> {
    let t_map = c_manager.get_table_map(db_name, true)?;
    let mut t_oid = 0;
    {
        let mut t_map = t_map.write().map_err(|e| e.to_string())?;
        if let CatalogData::TableMap(t_map) = &mut *t_map {
            let t_entry = t_map.get_table(table.trim());
            if t_entry.is_none() {
                return Err(DatabaseStatus::TableNotFound.message().to_string());
            }
            t_oid = t_entry.unwrap().oid();
            let c_map = c_manager.get_column_map(db_name, true, t_entry.unwrap())?;
            let mut c_map = c_map.write().map_err(|e| e.to_string())?;
            if let CatalogData::ColumnMap(c_map) = &mut *c_map {
                let entry = c_map.get_column_as_mut(column);
                if entry.is_none() {
                    return Err(DatabaseStatus::ColumnNotFound.message().to_string());
                }
                let entry = entry.unwrap();
                entry.make_primary();
                entry.make_not_nullable();
            }
        }
    }
    c_buffer.force_flush(db_name, CatalogType::ColumnMap, t_oid as u32)?;

    Ok(())
}
