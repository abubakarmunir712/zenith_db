use std::sync::Arc;

use crate::{
    enums::{
        errors::db_status::DatabaseStatus,
        types::catalog_types::{CatalogData, CatalogType},
    },
    indexing::Hashing::hash_bucket_manager::HashBucketManager,
    storage::{
        buffer::{
            catalog_buffer::CatalogBuffer,
            index_buffer::{self, IndexBuffer},
        },
        catalog::{catalog_manager::CatalogManager, maps::column_map::ColumnMap}, io::file_io::IOEngine,
    },
};

// pub fn make_primary(
//     db_name: &str,
//     column: &str,
//     table: &str,
//     c_manager: &CatalogManager,
//     c_buffer: &Arc<CatalogBuffer>,
//     index_buffer: &Arc<IndexBuffer>,
// ) -> Result<(), String> {
//     let t_map = c_manager.get_table_map(db_name, true)?;
//     let mut t_oid = 0;
//     {
//         let mut t_map = t_map.write().map_err(|e| e.to_string())?;
//         if let CatalogData::TableMap(t_map) = &mut *t_map {
//             let t_entry = t_map.get_table(table.trim());
//             if t_entry.is_none() {
//                 return Err(DatabaseStatus::TableNotFound.message().to_string());
//             }
//             t_oid = t_entry.unwrap().oid();
//             let c_map = c_manager.get_column_map(db_name, true, t_entry.unwrap())?;
//             let mut c_map = c_map.write().map_err(|e| e.to_string())?;
//             // let cmap: ColumnMap = ColumnMap::new();
//             if let CatalogData::ColumnMap(c_map) = &mut *c_map {
//                 let entry = c_map.get_column_as_mut(column);
//                 if entry.is_none() {
//                     return Err(DatabaseStatus::ColumnNotFound.message().to_string());
//                 }
//                 let entry = entry.unwrap();
//                 entry.make_primary();
//                 entry.make_not_nullable();
//                 let col_name = entry.column_name();
//                 drop(entry);
//                 HashBucketManager::create_index(
//                     db_name,
//                     t_oid as u32,
//                     &col_name,
//                     &c_map.clone(),
//                     &index_buffer,
//                 );
//             }
//         }
//     }
//     c_buffer.force_flush(db_name, CatalogType::ColumnMap, t_oid as u32)?;

//     Ok(())
// }


pub fn make_primary(
    db_name: &str,
    column: &str,
    table: &str,
    c_manager: &CatalogManager,
    c_buffer: &Arc<CatalogBuffer>,
    index_buffer: &Arc<IndexBuffer>,
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

            let t_entry = t_entry.unwrap();
            t_oid = t_entry.oid();

            let c_map = c_manager.get_column_map(db_name, true, t_entry)?;
            let mut c_map = c_map.write().map_err(|e| e.to_string())?;

            if let CatalogData::ColumnMap(c_map) = &mut *c_map {
                let entry = c_map.get_column_as_mut(column);
                if entry.is_none() {
                    return Err(DatabaseStatus::ColumnNotFound.message().to_string());
                }

                let entry = entry.unwrap();
                entry.make_primary();
                entry.make_not_nullable();

                // Extract column name before mutable borrow ends
                let col_name = entry.column_name().to_owned();

                // Now it's safe to immutably borrow c_map
                HashBucketManager::create_index(
                    db_name,
                    t_oid as u32,
                    &col_name,
                    c_map, // immutable borrow
                    index_buffer,
                )?;

            }
        }
    }
    c_buffer.force_flush(db_name, CatalogType::ColumnMap, t_oid as u32)?;

    Ok(())
}