/// Database configuration constants.
///
/// # Constants
/// - `MAX_TABLES_LIMIT`: Maximum number of tables allowed in the database.
/// - `MAX_COLUMNS_LIMIT`: Maximum number of columns allowed per table.
/// - `MAX_REF_SIZE`: Maximum number of foreign key references that can be stored in a single catalog page.
///
pub mod DbConfigs {
    pub const MAX_TABLES_LIMIT: usize = 400;
    pub const MAX_COLUMNS_LIMIT: usize = 400;
    pub const MAX_REF_SIZE: usize = 1000;
    pub const MAX_FOREGIN_KEYS_CONSTRAINTS: usize = 20;
    pub const MAX_COLUMNS_IN_FOREIGN_KEY: usize = 4;
    pub const MAX_COLUMNS_IN_PRIMARY_KEY: usize = 4;
    pub const MAX_TABLE_NAME_LENGTH:usize = 63;
    pub const MAX_COLUMN_NAME_LENGTH:usize = 63;
}
