use std::sync::Arc;

use crate::{
    configs::config::Config::PAGE_SIZE,
    enums::{
        errors::db_status::DatabaseStatus,
        types::{catalog_types::CatalogData, page_types::PageType},
    },
    indexing::Hashing::{bucket_value::BucketValue, hash_bucket_manager::HashBucketManager},
    storage::{
        buffer::{
            index_buffer::{self, IndexBuffer},
            page_buffer::PageBuffer,
        },
        catalog::catalog_manager::CatalogManager,
        io::file_io::IOEngine,
        page::page::Page,
        record::{record::Record, record_manager::RecordManager},
    },
};

pub fn insert_record(
    db_name: &str,
    table: &str,
    columns: Vec<String>,
    values: Vec<String>,
    c_manager: &CatalogManager,
    p_buffer: &Arc<PageBuffer>,
    index_buffer: &Arc<IndexBuffer>,
) -> Result<(), String> {
    let t_map = c_manager.get_table_map(db_name, true)?;
    let t_map = t_map.read().map_err(|e| e.to_string())?;
    let mut primary = String::new();
    let mut p_idx = 0;
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
            let mut r_cols: Vec<String> = Vec::new();
            for col in &columns {
                if c_map.get_column(&col).is_none() {
                    return Err(DatabaseStatus::ColumnNotFound.message().to_string());
                }
            }
            for col in c_map.ord_map() {
                let col_e = c_map.get_column(&col).unwrap();
                if col_e.is_primary_key() {
                    primary = col.to_string();
                }
                // println!("{}", col_e.is_primary_key());
                p_idx = r_cols.len();
                let idx = columns.iter().position(|x| x == col);
                if idx.is_none() {
                    r_cols.push("".to_string());
                } else {
                    let idx = idx.unwrap();
                    r_cols.push(values[idx].to_string());
                }
            }

            // if primary != ""
            //     && HashBucketManager::does_key_exists(
            //         &r_cols[p_idx],
            //         db_name,
            //         t_oid as u32,
            //         &primary,
            //         c_map,
            //         p_buffer,
            //         &index_buffer,
            //     )?
            // {
            //     return Err("Key already exixts".to_string());
            // }

            let record = Record::new(r_cols.clone(), c_map)?;

            pg_no =
                IOEngine::calculate_total_pages(db_name, &t_oid.to_string(), PageType::DataPage)?
                    - 1;
            let page = p_buffer.get_page(db_name, &t_oid.to_string(), pg_no, true)?;
            let mut page = page.write().unwrap();
            let mut i_res = RecordManager::insert_record(&record.serialize(), &mut page);
            if i_res.is_none() {
                let mut page = Page::new(0, 0);
                i_res = RecordManager::insert_record(&record.serialize(), &mut page);
                let mut page_buffer = [0; PAGE_SIZE as usize];
                page.serialize(&mut page_buffer);
                IOEngine::append_page(
                    db_name,
                    &t_oid.to_string(),
                    &page_buffer,
                    PageType::DataPage,
                )?;
                pg_no += 1;
            }
            let b = BucketValue::new(pg_no, i_res.unwrap().1);
            // if primary != "" {
            //     HashBucketManager::add_value(
            //         &r_cols[p_idx],
            //         &b,
            //         db_name,
            //         &format!("{}_{}", t_oid, primary),
            //         0,
            //         &index_buffer,
            //     )?;
            // }
        }
        p_buffer.force_flush(db_name, &t_oid.to_string(), pg_no)?;
    }
    Ok(())
}
