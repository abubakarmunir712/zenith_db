use std::sync::Arc;

use crate::{
    enums::{
        errors::db_status::DatabaseStatus,
        types::{
            catalog_types::{CatalogData, CatalogType},
            page_types::PageType,
        },
    },
    storage::{
        buffer::catalog_buffer::CatalogBuffer, catalog::catalog_manager::CatalogManager,
        io::file_io::IOEngine,
    },
};

pub fn delete_table(
    db_name: &str,
    table: &str,
    c_manager: &CatalogManager,
    c_buffer: &Arc<CatalogBuffer>,
) -> Result<(), String> {
    let table_map = c_manager.get_table_map(db_name, true)?;
    let mut t_oid = 0;
    {
        let mut table_map = table_map.write().map_err(|e| e.to_string())?;
        if let CatalogData::TableMap(t_map) = &mut *table_map {
            let t_entry = t_map.get_table(table.trim());
            if t_entry.is_none() {
                return Err(DatabaseStatus::TableNotFound.message().to_string());
            }
            t_oid = t_entry.unwrap().oid();
            CatalogManager::delete_table(table, t_map);
        }
    }
    c_buffer.force_flush(db_name, CatalogType::TableMap, 0)?;
    IOEngine::delete_table(db_name, &t_oid.to_string())?;
    Ok(())
}
