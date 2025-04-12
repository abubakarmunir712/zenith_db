use crate::types::char::CHAR;
use crate::types::bool::BOOL;
use crate::types::int::INT;
use crate::types::date::DATE;
use crate::types::time::TIME;
use crate::types::date_time::DATETIME;
use crate::storage::catalog::ColumnInfo;

pub enum DataType {
    CHAR(CHAR),
    VARCHAR(CHAR),
    BOOL(BOOL),
    INT(INT),
    DATE(DATE),
    TIME(TIME),
    DATETIME(DATETIME),
}

impl DataType {
    pub fn to_bytes(&self, column_info: &ColumnInfo) -> Result<Vec<u8>, String> {
        match (self, &column_info.data_type) {
            (DataType::CHAR(c), DataType::CHAR(_)) => Ok(c.to_bytes(true)),
            (DataType::VARCHAR(v), DataType::VARCHAR(_)) => Ok(v.to_bytes(false)),
            (DataType::BOOL(b), DataType::BOOL(_)) => Ok(b.to_bytes().to_vec()),
            (DataType::INT(i), DataType::INT(_)) => Ok(i.to_bytes().to_vec()),
            (DataType::DATE(d), DataType::DATE(_)) => Ok(d.to_bytes().to_vec()),
            (DataType::TIME(t), DataType::TIME(_)) => Ok(t.to_bytes().to_vec()),
            (DataType::DATETIME(dt), DataType::DATETIME(_)) => Ok(dt.to_bytes().to_vec()),
            _ => Err("Mismatched DataType and ColumnInfo".to_string()),
        }
    }

    pub fn from_bytes(data: &[u8], column_info: &ColumnInfo) -> Result<Self, String> {
        match &column_info.data_type {
            DataType::CHAR(_) => {
                let value = std::str::from_utf8(data).map_err(|_| "Invalid CHAR bytes")?.trim_end();
                Ok(DataType::CHAR(CHAR::new(column_info.max_data_size, value).unwrap())) //CHECK THIS CODE "?" missing
            }
            DataType::VARCHAR(_) => {
                let value = std::str::from_utf8(data).map_err(|_| "Invalid VARCHAR bytes")?;
                Ok(DataType::VARCHAR(CHAR::new(column_info.max_data_size, value).unwrap())) //CHECK THIS CODE "?" missing
            }
            DataType::BOOL(_) => {
                Ok(DataType::BOOL(BOOL::from_bytes(&[data[0]])))
            }
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
                Ok(DataType::DATETIME(DATETIME::from_bytes(&data.try_into().unwrap())))
            }
        }
    }
    
}
