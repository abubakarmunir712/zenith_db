use crate::enums::datatypes::DataType;

pub struct ColumnEntry {
    pub column_name: String,
    pub oid: u16,
    pub datatype:DataType,
    pub max_size:u32,
    pub null: bool,
    pub unique: bool,
    pub is_primary_key: bool,
    pub is_foreign_key:bool,
    pub is_referenced:bool,
}
