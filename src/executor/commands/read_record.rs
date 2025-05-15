use std::sync::Arc;

use crate::{
    configs::config::Config::PAGE_SIZE,
    enums::{
        errors::db_status::DatabaseStatus,
        types::{catalog_types::CatalogData, page_types::PageType},
    },
    storage::{
        buffer::page_buffer::PageBuffer,
        catalog::catalog_manager::CatalogManager,
        io::file_io::IOEngine,
        page::page::Page,
        record::{record::Record, record_manager::RecordManager},
    },
};

pub fn read_record(
    db_name: &str,
    table: &str,
    c_manager: &CatalogManager,
    p_buffer: &Arc<PageBuffer>,
) -> Result<(Vec<String>, Vec<Record>), String> {
    let mut records = Vec::new();
    let mut col_names = Vec::new();
    let t_map = c_manager.get_table_map(db_name, true)?;
    let t_map = t_map.read().map_err(|e| e.to_string())?;
    let mut pg_no = 0;
    if let CatalogData::TableMap(t_map) = &*t_map {
        let t_entry = t_map.get_table(table.trim());
        if t_entry.is_none() {
            return Err(DatabaseStatus::TableNotFound.message().to_string());
        }
        let t_oid = t_entry.unwrap().oid();
        let c_map = c_manager.get_column_map(db_name, true, t_entry.unwrap())?;
        let c_map = c_map.read().map_err(|e| e.to_string())?;
        if let CatalogData::ColumnMap(c_map) = &*c_map {
            c_map
                .ord_map()
                .iter()
                .for_each(|f| col_names.push(f.to_string()));
            for pg_no in
                0..IOEngine::calculate_total_pages(db_name, &t_oid.to_string(), PageType::DataPage)?
            {
                let page = p_buffer.get_page(db_name, &t_oid.to_string(), pg_no, false)?;
                let page = page.read().unwrap();
                let t_rec = RecordManager::read_records(&page, c_map);
                records.extend(t_rec);
            }
        }
    }
    Ok((col_names, records))
}
