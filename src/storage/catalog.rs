use crate::enums::datatypes::DataType;
use bincode;

//Catalog table stored as a struct
pub struct CatalogTable {
    pub table_name: String,
    pub database_name: String,
    pub no_of_fixed_columns: u16,
    pub no_of_variable_columns: u16,
    pub fixed_columns: Vec<ColumnInfo>,
    pub variable_columns: Vec<ColumnInfo>,
}
pub struct ColumnInfo {
    pub column_name: String,
    pub max_data_size: u32,
    pub data_type: DataType,
}
impl CatalogTable {
    pub fn new(
        table_name: String,
        database_name: String,
        fixed_columns: Vec<ColumnInfo>,
        variable_columns: Vec<ColumnInfo>,
    ) -> Self {

        let no_of_fixed_columns = fixed_columns.len() as u16;
        let no_of_variable_columns = variable_columns.len() as u16;

        Self {
            table_name,
            database_name,
            no_of_fixed_columns,
            no_of_variable_columns,
            fixed_columns,
            variable_columns,
        }
    }

    pub fn size(&self)->u16{
        self.no_of_fixed_columns+self.no_of_variable_columns
    }
}

impl ColumnInfo {
    pub fn new(column_name: String, max_data_size: u32, data_type: DataType) -> Self {
        Self {
            column_name,
            max_data_size,
            data_type,
        }
    }
}
impl ColumnInfo {
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


//The below code is for testing, will form proper tests later.

// fn main() -> bincode::Result<()>

// {
//     let catalog = CatalogTable {

//         table_name: "users".to_string(),
//         no_of_columns:2,
//         columns: vec![
//             ColumnInfo {
//                 column_name: "id".to_string(),
//                 max_data_size:4,
//                 data_type: DataType::Integer,
//             },
//             ColumnInfo {
//                 column_name: "username".to_string(),
//                 max_data_size:200,
//                 data_type: DataType::Varchar,
//             },

//         ],
//     };

//     // Serialize to binary
//     let encoded: Vec<u8> = bincode::serialize(&catalog)?;
//     println!("Serialized catalog ({} bytes)", encoded.len());

//     // Deserialize back
//     let decoded: CatalogTable = bincode::deserialize(&encoded)?;
//     println!("Deserialized catalog: {:#?}", decoded);

//    Ok(())
// }
