pub enum DatabaseStatus {
    DatabaseNotFound,
    FileNotFound,
    FileExistsInDatabase,
    DatabaseAlreadyExists,
    FileAlreadyExists,
    PageNotFoundInFile,
    TableAlreadyExists,
    ColumnNameUnique,
    TableNotFound,
    ColumnNotFound
}

impl DatabaseStatus {
    pub fn message(&self) -> &str {
        match self {
            DatabaseStatus::DatabaseNotFound => "Database not found",
            DatabaseStatus::FileNotFound => "File not found",
            DatabaseStatus::FileExistsInDatabase => "File already exists in database",
            DatabaseStatus::DatabaseAlreadyExists => "Database already exists",
            DatabaseStatus::FileAlreadyExists => "File already exists",
            DatabaseStatus::PageNotFoundInFile => "Page not found in file",
            DatabaseStatus::TableAlreadyExists=>"A table with this name already exists",
            DatabaseStatus::ColumnNameUnique=>"Column name must be unique",
            DatabaseStatus::TableNotFound=>"Table not found",
            DatabaseStatus::ColumnNotFound=>"Column not found",

        }
    }
}
