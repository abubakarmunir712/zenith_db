/// Errors related to catalog constraints.
#[derive(Debug)]
pub enum CatalogError {
    /// Occurs when the maximum foreign key constraints are exceeded.
    SysMaxFKConstraintExceeded,

    /// Occurs when the maximum columns in a foreign key are exceeded.
    SysMaxColumnsInFKExceeded,

    /// Occurs when the maximum columns in a primary key are exceeded.
    SysMaxColumnsInPKExceeded,

    /// Occurs when the maximum columns in a table are exceeded.
    SysMaxColumnsInTableExceeded,

    /// Occurs when the maximum number of tables in the database is exceeded.
    SysMaxTablesInDBLimitExceeded,

    /// Occurs when the length of the table name exceeds the allowed limit.
    TableNameLengthLimitExceeded,

    /// Occurs when the length of the column name exceeds the allowed limit.
    ColumnNameLengthLimitExceeded,

    /// Occurs when refernce page is full
    SysMaxRefPerPageLimitExceeded,
}

#[rustfmt::skip]
impl CatalogError {
    /// Returns a human-readable message describing the specific catalog error.
    pub fn message(&self) -> &str {
        match self {
            CatalogError::SysMaxFKConstraintExceeded => "Maximum foreign key constraints exceeded.",
            CatalogError::SysMaxColumnsInFKExceeded => "Maximum columns in a foreign key exceeded.",
            CatalogError::SysMaxColumnsInPKExceeded => "Maximum columns in a primary key exceeded.",
            CatalogError::SysMaxColumnsInTableExceeded => "Maximum columns in a table exceeded.",
            CatalogError::SysMaxTablesInDBLimitExceeded => "Maximum number of tables in the database exceeded.",
            CatalogError::TableNameLengthLimitExceeded => "Table name length limit exceeded.",
            CatalogError::ColumnNameLengthLimitExceeded => "Column name length limit exceeded.",
            CatalogError::SysMaxRefPerPageLimitExceeded=>"Maximum number of refernces in a page exceeded",
        }

    }
}
