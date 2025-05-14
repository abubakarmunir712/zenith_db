use crate::{configs::config::Config::CATLOG_PAGE_SIZE, enums::{errors::db_status::DatabaseStatus, types::page_types::PageType}, storage::{buffer, catalog::maps::table_map::TableMap, io::file_io::IOEngine}};

pub fn create_db(db_name: &str) -> Result<(), String> {
    if IOEngine::db_exists(db_name) {
        return Err(DatabaseStatus::DatabaseAlreadyExists.message().to_string());
    }
    IOEngine::create_db(db_name)?;
    let table_map = TableMap::new();
    let mut buffer = [0;CATLOG_PAGE_SIZE as usize];
    table_map.serialize(&mut buffer);
    IOEngine::append_page(db_name, db_name, &buffer, PageType::CatlogPage)?;
    Ok(())
}
