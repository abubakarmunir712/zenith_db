use crate::enums::types::datatypes::DataType;
use crate::storage::catalog::entries::column_entry::ColumnEntry;
use crate::types::big_int::BIGINT;
use crate::types::bool::BOOL;
use crate::types::char::CHAR;
use crate::types::date::DATE;
use crate::types::date_time::DATETIME;
use crate::types::decimal::DECIMAL;
use crate::types::double::DOUBLE;
use crate::types::float::FLOAT;
use crate::types::int::INT;
use crate::types::null::NULL;
use crate::types::small_int::SMALLINT;
use crate::types::text::TEXT;
use crate::types::time::TIME;
use crate::types::tiny_int::TINYINT;
use crate::types::varchar::VARCHAR;

pub enum TypedValue {
    CHAR(CHAR),         // Fixed-length character type
    VARCHAR(VARCHAR),   // Variable-length character type
    BOOL(BOOL),         // Boolean type (true/false)
    INT(INT),           // Integer type
    BIGINT(BIGINT),     // Big integer type
    SMALLINT(SMALLINT), // Small integer type
    TINYINT(TINYINT),   // Tiny integer type
    DECIMAL(DECIMAL),   // Decimal type (fixed-point number)
    DOUBLE(DOUBLE),     // Double precision floating-point number
    FLOAT(FLOAT),       // Single precision floating-point number
    DATE(DATE),         // Date type
    TIME(TIME),         // Time type
    DATETIME(DATETIME), // Combined Date and Time type
    TEXT(TEXT),         // Text type (longer string)
    NULL(NULL)          // Null Type
}

impl TypedValue {
    /// Converts the `TypedValue` to a byte array for storage.
    ///
    /// # Returns
    /// A `Vec<u8>` representing the byte array of the `TypedValue`.
    ///
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TypedValue::CHAR(c) => c.to_bytes(),
            TypedValue::VARCHAR(v) => v.to_bytes(),
            TypedValue::BOOL(b) => b.to_bytes(),
            TypedValue::INT(i) => i.to_bytes(),
            TypedValue::BIGINT(b) => b.to_bytes(),
            TypedValue::SMALLINT(s) => s.to_bytes(),
            TypedValue::TINYINT(t) => t.to_bytes(),
            TypedValue::DECIMAL(d) => d.to_bytes(),
            TypedValue::DOUBLE(d) => d.to_bytes(),
            TypedValue::FLOAT(f) => f.to_bytes(),
            TypedValue::DATE(d) => d.to_bytes(),
            TypedValue::TIME(t) => t.to_bytes(),
            TypedValue::DATETIME(dt) => dt.to_bytes(),
            TypedValue::TEXT(t) => t.to_bytes(),
            TypedValue::NULL(_)=>Vec::new()
        }
    }

    /// Creates a `TypedValue` from a byte array, based on the `ColumnEntry` data type.
    ///
    /// This function is used for deserializing data from a byte array into a specific data type,
    /// such as `CHAR`, `VARCHAR`, `INT`, etc., based on the column entry.
    ///
    /// # Arguments
    /// * `bytes` - A byte slice (`&[u8]`) containing the raw bytes representing the value.
    /// * `column_entry` - A `ColumnEntry` that contains the datatype information for proper deserialization.
    ///
    /// # Returns
    /// A `TypedValue` enum variant corresponding to the appropriate data type.
    ///
    #[rustfmt::skip]
    pub fn from_bytes(bytes: &[u8], column_entry: &ColumnEntry) -> Self {
        match &column_entry.datatype() {
            DataType::CHAR => TypedValue::CHAR(CHAR::from_bytes(bytes, column_entry.max_size())),
            DataType::VARCHAR => TypedValue::VARCHAR(VARCHAR::from_bytes(bytes, column_entry.max_size())),
            DataType::BOOL => TypedValue::BOOL(BOOL::from_bytes(bytes)),
            DataType::INT => TypedValue::INT(INT::from_bytes(bytes)),
            DataType::BIGINT => TypedValue::BIGINT(BIGINT::from_bytes(bytes)),
            DataType::SMALLINT => TypedValue::SMALLINT(SMALLINT::from_bytes(bytes)),
            DataType::TINYINT => TypedValue::TINYINT(TINYINT::from_bytes(bytes)),
            DataType::DECIMAL => TypedValue::DECIMAL(DECIMAL::from_bytes(bytes,column_entry.max_size() / 100,column_entry.max_size() % 100,)),
            DataType::DOUBLE => TypedValue::DOUBLE(DOUBLE::from_bytes(bytes)),
            DataType::FLOAT => TypedValue::FLOAT(FLOAT::from_bytes(bytes)),
            DataType::DATE => TypedValue::DATE(DATE::from_bytes(bytes)),
            DataType::TIME => TypedValue::TIME(TIME::from_bytes(bytes)),
            DataType::DATETIME => TypedValue::DATETIME(DATETIME::from_bytes(bytes)),
            DataType::TEXT => TypedValue::TEXT(TEXT::from_bytes(bytes)),
            DataType::NULL=>TypedValue::NULL(NULL::new())
        }
        
    }

    pub fn new(value:&str, column_entry: &ColumnEntry)->Result<Self,String>{
        let typevalue = match &column_entry.datatype(){
            DataType::CHAR =>TypedValue::CHAR(CHAR::new(column_entry.max_size(), value)?),
            DataType::VARCHAR => TypedValue::VARCHAR(VARCHAR::new(column_entry.max_size(), value)?),
            DataType::BOOL => TypedValue::BOOL(BOOL::new( value)?),
            DataType::INT =>  TypedValue::INT(INT::new( value)?),
            DataType::BIGINT =>  TypedValue::BIGINT(BIGINT::new( value)?),
            DataType::SMALLINT =>  TypedValue::SMALLINT(SMALLINT::new( value)?),
            DataType::TINYINT =>  TypedValue::TINYINT(TINYINT::new( value)?),
            DataType::DECIMAL =>  TypedValue::DECIMAL(DECIMAL::new( value,column_entry.max_size() / 100,column_entry.max_size() % 100)?),
            DataType::DOUBLE => TypedValue::DOUBLE(DOUBLE::new(value)?),
            DataType::FLOAT => TypedValue::FLOAT(FLOAT::new(value)?),
            DataType::DATE => TypedValue::DATE(DATE::new(value)?),
            DataType::TIME =>TypedValue::TIME(TIME::new(value)?),
            DataType::DATETIME =>TypedValue::DATETIME(DATETIME::new(value)?),
            DataType::TEXT => TypedValue::TEXT(TEXT::new(value)?),
            DataType::NULL=>TypedValue::NULL(NULL::new())
        };
        Ok(typevalue)
    }

    pub fn to_string(&self)->String{
        match self{
            TypedValue::CHAR(c) => c.value().to_string(),
            TypedValue::VARCHAR(v) => v.value().to_string(),
            TypedValue::BOOL(b) => b.value().to_string(),
            TypedValue::INT(i) => i.value().to_string(),
            TypedValue::BIGINT(b) => b.value().to_string(),
            TypedValue::SMALLINT(s) => s.value().to_string(),
            TypedValue::TINYINT(t) => t.value().to_string(),
            TypedValue::DECIMAL(d) => d.value_string(),
            TypedValue::DOUBLE(d) => d.value().to_string(),
            TypedValue::FLOAT(f) => f.value().to_string(),
            TypedValue::DATE(d) => d.value(),
            TypedValue::TIME(t) => t.value(),
            TypedValue::DATETIME(dt) => dt.value(),
            TypedValue::TEXT(t) => t.value().to_string(),
            TypedValue::NULL(_)=>"Null".to_string()
        }

    }
}
