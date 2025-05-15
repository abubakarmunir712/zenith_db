use std::sync::Arc;

use crate::{
    enums::{
        commands::cmd::SqlCommand,
        types::res_type::{ResType, View},
    },
    storage::{
        buffer::{
            catalog_buffer::CatalogBuffer, index_buffer::IndexBuffer, page_buffer::PageBuffer,
        },
        catalog::catalog_manager::CatalogManager,
    },
};

use super::commands::{
    create_db::create_db, delete_db::delete_db, delete_record::delete_record,
    delete_table::delete_table, insert_record::insert_record, insert_table::insert_table,
    make_null::make_null, make_primary::make_primary, read_record::read_record,
    select_by_val::select_by_val,
};

pub struct Executor {
    c_manager: CatalogManager,
    c_buffer: Arc<CatalogBuffer>,
    p_buffer: Arc<PageBuffer>,
    i_buffer: Arc<IndexBuffer>,
}

impl Executor {
    pub fn new() -> Arc<Self> {
        let c_buffer = CatalogBuffer::new();
        let c_manager = CatalogManager {
            catlog_buffer: c_buffer.clone(),
        };
        let i_buffer = IndexBuffer::new();
        let p_buffer = PageBuffer::new();
        Arc::new(Self {
            c_manager,
            c_buffer,
            p_buffer,
            i_buffer,
        })
    }

    pub fn execute(&self, command: SqlCommand) -> ResType {
        match command {
            SqlCommand::CreateDB { db_name } => match create_db(&db_name) {
                Ok(_) => {
                    return ResType::Success("Database created successfully!".to_string());
                }
                Err(e) => {
                    return ResType::Error(format!("{}", e));
                }
            },
            SqlCommand::CreateTable {
                db_name,
                table,
                columns,
            } => match insert_table(&db_name, &table, columns, &self.c_manager, &self.c_buffer) {
                Ok(_) => {
                    return ResType::Success("Table created successfully!".to_string());
                }
                Err(e) => {
                    return ResType::Error(format!("Error: {}", e));
                }
            },
            SqlCommand::DeleteDB { db_name } => match delete_db(&db_name) {
                Ok(_) => {
                    return ResType::Success("Database deleted successfully!".to_string());
                }
                Err(e) => {
                    return ResType::Error(format!("{}", e));
                }
            },
            SqlCommand::DeleteTable { db_name, table } => {
                match delete_table(&db_name, &table, &self.c_manager, &self.c_buffer) {
                    Ok(_) => {
                        return ResType::Success("Table deleted successfully!".to_string());
                    }
                    Err(e) => {
                        return ResType::Error(format!("{}", e));
                    }
                }
            }
            SqlCommand::MakeNull {
                db_name,
                table,
                column,
            } => match make_null(&db_name, &column, &table, &self.c_manager, &self.c_buffer) {
                Ok(_) => {
                    return ResType::Success("Made nullable successfully".to_string());
                }
                Err(e) => {
                    return ResType::Error(format!("Error: {}", e));
                }
            },
            SqlCommand::MakePrimary {
                db_name,
                table,
                column,
            } => match make_primary(
                &db_name,
                &column,
                &table,
                &self.c_manager,
                &self.c_buffer,
                &self.i_buffer,
            ) {
                Ok(_) => {
                    return ResType::Success("Made primary successfully".to_string());
                }
                Err(e) => {
                    return ResType::Error(format!("Error: {}", e));
                }
            },
            SqlCommand::Insert {
                db_name,
                table,
                columns,
                values,
            } => match insert_record(
                &db_name,
                &table,
                columns,
                values,
                &self.c_manager,
                &self.p_buffer,
            ) {
                Ok(_) => {
                    return ResType::Success("Record inserted successfully".to_string());
                }
                Err(e) => {
                    return ResType::Error(format!("{}", e));
                }
            },
            SqlCommand::Select {
                db_name,
                all,
                table,
                filter,
            } => match read_record(&db_name, &table, &self.c_manager, &self.p_buffer) {
                Ok(r) => {
                    return ResType::View(View::new(r.0, r.1));
                }
                Err(e) => {
                    return ResType::Error(format!("{}", e));
                }
            },
            SqlCommand::SelectByValue {
                db_name,
                col,
                table,
                value,
            } => match select_by_val(
                &db_name,
                &table,
                &col,
                &value,
                &self.c_manager,
                &self.p_buffer,
            ) {
                Ok(r) => {
                    return ResType::View(View::new(r.0, r.1));
                }
                Err(e) => {
                    return ResType::Error(format!("{}", e));
                }
            },
            SqlCommand::DeleteRecord {
                db_name,
                col,
                table,
                value,
            } => match delete_record(
                &db_name,
                &table,
                &col,
                &value,
                &self.c_manager,
                &self.p_buffer,
            ) {
                Ok(r) => {
                    return ResType::Success("Record deleted successfully!".to_string());
                }
                Err(e) => {
                    return ResType::Error(format!("{}", e));
                }
            },
            (_) => return ResType::Success("Method not implemented".to_string()),
        }
    }
}
