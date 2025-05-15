use super::cmd_obj::{ColumnDef, Condition};

pub enum SqlCommand {
    CreateDB {
        db_name: String,
    },
    DeleteDB {
        db_name: String,
    },
    CreateTable {
        db_name: String,
        table: String,
        columns: Vec<ColumnDef>,
    },
    DeleteTable {
        db_name: String,
        table: String,
    },
    Insert {
        db_name: String,
        table: String,
        columns: Vec<String>,
        values: Vec<String>,
    },
    Select {
        db_name: String,
        table: String,
        filter: Option<Condition>,
        all: bool,
    },

    SelectByValue {
        db_name: String,
        table: String,
        value: String,
        col: String,
    },
    MakeNull {
        db_name: String,
        table: String,
        column: String,
    },
    MakePrimary {
        db_name: String,
        table: String,
        column: String,
    },
    DeleteRecord {
        db_name: String,
        table: String,
        value: String,
        col: String,
    }
}
