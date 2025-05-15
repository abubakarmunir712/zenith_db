use std::sync::{Arc, RwLock};

use crate::{
    configs::config::Config::{CATLOG_PAGE_SIZE, PAGE_SIZE},
    enums::{
        commands::cmd_obj::ColumnDef,
        errors::db_status::DatabaseStatus,
        types::{
            catalog_types::{CatalogData, CatalogType},
            page_types::PageType,
        },
    },
    storage::{
        buffer::catalog_buffer::CatalogBuffer,
        catalog::{catalog_manager::CatalogManager, maps::column_map::ColumnMap},
        io::file_io::IOEngine,
        page::page::Page,
    },
};

pub fn insert_table(
    db_name: &str,
    table: &str,
    cols: Vec<ColumnDef>,
    c_manager: &CatalogManager,
    c_buffer: &Arc<CatalogBuffer>,
) -> Result<(), String> {
    let t_map: Arc<RwLock<CatalogData>> = c_manager.get_table_map(db_name, true)?;
    let mut t_oid = 0;
    {
        let mut t_maps = t_map.write().map_err(|e| e.to_string())?;

        if let CatalogData::TableMap(map) = &mut *t_maps {
            if map.get_table(table).is_some() {
                return Err(DatabaseStatus::TableAlreadyExists.message().to_string());
            }
            t_oid = CatalogManager::create_table(table, map)?;
            let mut buffer = [0; CATLOG_PAGE_SIZE as usize];
            ColumnMap::new().serialize(&mut buffer);
            IOEngine::append_page(db_name, db_name, &buffer, PageType::CatlogPage)?;
            let t_entry = map.get_table_as_mut(table).unwrap();
            let column_map = c_manager.get_column_map(db_name, true, t_entry)?;
            {
                let mut column_map = column_map.write().map_err(|e| e.to_string())?;
                if let CatalogData::ColumnMap(c_map) = &mut *column_map {
                    for col in cols {
                        let res = c_manager.create_column(
                            &col.name,
                            t_entry,
                            c_map,
                            col.datatype,
                            col.size,
                        );
                        if res.is_err() {
                            c_buffer.force_evict(db_name, CatalogType::TableMap, 0)?;
                            c_buffer.force_evict(db_name, CatalogType::ColumnMap, t_oid as u32)?;
                            res?;
                        }
                    }
                }
            }
        }
    }
    c_buffer.force_flush(db_name, CatalogType::TableMap, 0)?;
    c_buffer.force_flush(db_name, CatalogType::ColumnMap, t_oid as u32)?;
    IOEngine::create_table(db_name, &t_oid.to_string())?;
    let page = Page::new(0, 0);
    let mut buffer = [0; PAGE_SIZE as usize];
    page.serialize(&mut buffer);
    IOEngine::append_page(db_name, &t_oid.to_string(), &buffer, PageType::DataPage);

    Ok(())
}
