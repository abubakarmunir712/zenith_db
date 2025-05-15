pub enum ParserError{
    InvalidSyntax,
    ColumnNamesRequired,
    InvalidColumnName,
    DataTypeRequired,
    LenghtRequired,
    DBInUse
}

impl ParserError{
    pub fn message(&self)->&str{
        match &self {
            ParserError::InvalidSyntax=>"Error: Invalid syntax",
            ParserError::ColumnNamesRequired=>"Error: Expected columns after table name",
            ParserError::InvalidColumnName=>"Column name cannot contain ':'",
            ParserError::DataTypeRequired=>"Data Type is required",
            ParserError::LenghtRequired=>"Length required for char and varchar",
            ParserError::DBInUse=>"Database is in use",
        }
    }
}