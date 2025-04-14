// Importing custom types for each supported data type.
use crate::storage::catalog::catalog::ColumnInfo;
use crate::types::bool::BOOL;
use crate::types::char::CHAR;
use crate::types::date::DATE;
use crate::types::date_time::DATETIME;
use crate::types::int::INT;
use crate::types::time::TIME;
use crate::types::varchar::VARCHAR;

/// Enum representing all supported data types in the system.
/// Each variant wraps a corresponding custom type implementation.
pub enum DataType {
    CHAR(CHAR),         // Fixed-length character type
    VARCHAR(VARCHAR),      // Variable-length character type
    BOOL(BOOL),         // Boolean type (true/false)
    INT(INT),           // Integer type
    DATE(DATE),         // Date type
    TIME(TIME),         // Time type
    DATETIME(DATETIME), // Combined Date and Time type
}

impl DataType {
    /// Serializes the `DataType` instance into a byte vector based on the `ColumnInfo`.
    ///
    /// # Arguments
    /// * `column_info` - Metadata about the column, used to validate matching types.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Serialized byte representation if types match.
    /// * `Err(String)` - Error message if there's a mismatch between value and column type.
    pub fn to_bytes(&self, column_info: &ColumnInfo) -> Result<Vec<u8>, String> {
        match (self, &column_info.data_type) {
            // Match DataType variants and serialize accordingly
            (DataType::CHAR(c), DataType::CHAR(_)) => Ok(c.to_bytes()),
            (DataType::VARCHAR(v), DataType::VARCHAR(_)) => Ok(v.to_bytes()),
            (DataType::BOOL(b), DataType::BOOL(_)) => Ok(b.to_bytes().to_vec()),
            (DataType::INT(i), DataType::INT(_)) => Ok(i.to_bytes().to_vec()),
            (DataType::DATE(d), DataType::DATE(_)) => Ok(d.to_bytes().to_vec()),
            (DataType::TIME(t), DataType::TIME(_)) => Ok(t.to_bytes().to_vec()),
            (DataType::DATETIME(dt), DataType::DATETIME(_)) => Ok(dt.to_bytes().to_vec()),
            // Return error if types do not match
            _ => Err("Mismatched DataType and ColumnInfo".to_string()),
        }
    }

    /// Deserializes a byte slice into a `DataType` instance based on the `ColumnInfo`.
    ///
    /// # Arguments
    /// * `data` - Byte slice representing the raw data.
    /// * `column_info` - Metadata about the column, used to determine which type to deserialize into.
    ///
    /// # Returns
    /// * `Ok(DataType)` - Deserialized `DataType` object if successful.
    /// * `Err(String)` - Error message if deserialization fails or types are incompatible.
    ///
    pub fn from_bytes(data: &[u8], column_info: &ColumnInfo) -> Result<Self, String> {
        match &column_info.data_type {
            DataType::CHAR(_) => {
                let value = CHAR::from_bytes(data.to_vec(), column_info.max_data_size).unwrap();
                Ok(DataType::CHAR(CHAR::new(column_info.max_data_size, value.value()).unwrap())) //CHECK THIS CODE "?" missing
            }
            DataType::VARCHAR(_) => {
                let value = VARCHAR::from_bytes(data.to_vec(), column_info.max_data_size);
                Ok(DataType::VARCHAR(value.unwrap())) //CHECK THIS CODE "?" missing
            }
            DataType::BOOL(_) => Ok(DataType::BOOL(BOOL::from_bytes(&[data[0]]))),
            DataType::INT(_) => {
                if data.len() != 4 {
                    return Err("Invalid INT size".into());
                }
                Ok(DataType::INT(INT::from_bytes(data.try_into().unwrap())))
            }
            DataType::DATE(_) => {
                if data.len() != 4 {
                    return Err("Invalid DATE size".into());
                }
                Ok(DataType::DATE(DATE::from_bytes(&data.try_into().unwrap())))
            }
            DataType::TIME(_) => {
                if data.len() != 3 {
                    return Err("Invalid TIME size".into());
                }
                Ok(DataType::TIME(TIME::from_bytes(&data.try_into().unwrap())))
            }
            DataType::DATETIME(_) => {
                if data.len() != 7 {
                    return Err("Invalid DATETIME size".into());
                }
                Ok(DataType::DATETIME(DATETIME::from_bytes(
                    &data.try_into().unwrap(),
                )))
            }
        }
    }
}
