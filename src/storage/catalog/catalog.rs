use crate::enums::datatypes::DataType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct CatalogTable {
    pub table_name: String,            // Name of the table
    pub database_name: String,          // Name of the database the table belongs to
    pub no_of_fixed_columns: u16,       // Number of fixed-length columns
    pub no_of_variable_columns: u16,    // Number of variable-length columns
    pub fixed_columns: Vec<ColumnInfo>, // Metadata of fixed-length columns
    pub variable_columns: Vec<ColumnInfo>, // Metadata of variable-length columns
    pub is_dirty: bool,
}

// ColumnInfo struct represents metadata for an individual column.

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ColumnInfo {
    pub column_name: String,    // Name of the column
    pub max_data_size: u32,     // Maximum size of data stored in the column (in bytes)
    pub data_type: DataType,    // Data type of the column
}

impl CatalogTable {
    /// Creates a new instance of CatalogTable.
    /// 
    /// # Arguments
    /// * `table_name` - Name of the table.
    /// * `database_name` - Name of the database.
    /// * `fixed_columns` - List of fixed-length columns.
    /// * `variable_columns` - List of variable-length columns.
    pub fn new(
        table_name: String,
        database_name: String,
        fixed_columns: Vec<ColumnInfo>,
        variable_columns: Vec<ColumnInfo>,
    ) -> Self {

        // Calculate number of fixed and variable columns
        let no_of_fixed_columns = fixed_columns.len() as u16;
        let no_of_variable_columns = variable_columns.len() as u16;
        let is_dirty:bool = false;

        // Return a new CatalogTable instance
        Self {
            table_name,
            database_name,
            no_of_fixed_columns,
            no_of_variable_columns,
            fixed_columns,
            variable_columns,
            is_dirty
            
        }
    }

    /// Returns the total number of columns (fixed + variable) in the table.
    pub fn size(&self) -> u16 {
        self.no_of_fixed_columns + self.no_of_variable_columns
    }
}

impl ColumnInfo {
    /// Creates a new instance of ColumnInfo.
    /// 
    /// # Arguments
    /// * `column_name` - Name of the column.
    /// * `max_data_size` - Maximum data size in bytes.
    /// * `data_type` - Data type of the column.
    pub fn new(column_name: String, max_data_size: u32, data_type: DataType) -> Self {
        Self {
            column_name,
            max_data_size,
            data_type,
        }
    }
}

impl ColumnInfo {
        /// Calculates and returns the size of the column based on its data type.
    pub fn size(&self) -> usize {
        match self.data_type {
            DataType::CHAR(_) => self.max_data_size as usize,
            DataType::VARCHAR(_) => self.max_data_size as usize,
            DataType::BOOL(_) => 1,
            DataType::INT(_) => 4,
            DataType::DATE(_) => 4,
            DataType::TIME(_) => 3,
            DataType::DATETIME(_) => 7,
        }
    }
}
impl CatalogTable {
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn set_dirty(&mut self, value: bool) {
        self.is_dirty = value;
    }
}
