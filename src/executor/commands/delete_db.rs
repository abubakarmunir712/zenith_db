use crate::{enums::errors::db_status::DatabaseStatus, storage::io::file_io::IOEngine};

pub fn delete_db(db_name: &str) -> Result<(), String> {
    if !IOEngine::db_exists(db_name) && db_name != "root" {
        return Err(DatabaseStatus::DatabaseNotFound.message().to_string());
    }
    IOEngine::delete_db(db_name)?;
    Ok(())
}
