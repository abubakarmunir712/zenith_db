use crate::enums::errors::type_errors::ValidationError;
use crate::enums::types::typed_value::TypedValue;
use crate::storage::catalog::maps::column_map::ColumnMap;
use crate::types::null::NULL;

/// A `Record` represents a single data unit consisting of fixed and variable size fields.
///
/// The structure supports serialization to and deserialization from a raw byte buffer.
/// It uses little-endian byte order for encoding numeric fields.
pub struct Record {
    col_length: Vec<u16>,
    columns: Vec<TypedValue>,
}

impl Record {
    pub fn new(cols: Vec<String>, column_map: &ColumnMap) -> Result<Self, String> {
        let mut columns: Vec<TypedValue> = Vec::new();
        let mut col_length: Vec<u16> = Vec::new();
        for (index, col) in cols.iter().enumerate() {
            let col_name = &column_map.ord_map()[index];
            let column_entry = column_map.get_column(col_name).unwrap();
            if col == "" {
                if column_entry.is_nullable() {
                    columns.push(TypedValue::NULL(NULL::new()));
                    col_length.push(0);
                    continue;
                } else {
                    return Err(ValidationError::CannotBeNull.message().to_string());
                }
            }
            let col_val = TypedValue::new(&col, column_entry)?;
            let col_len = col_val.to_bytes().len() as u16;
            columns.push(col_val);
            col_length.push(col_len);
        }
        Ok(Record {
            columns,
            col_length,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Write each col_length as 2 bytes (u16)
        for &len in &self.col_length {
            result.extend(&len.to_be_bytes()); // Big-endian
        }

        // Append bytes of each column's value
        for val in &self.columns {
            result.extend(val.to_bytes());
        }

        result
    }

    pub fn deserialize(bytes: &[u8], column_map: &ColumnMap) -> Self {
        let mut columns: Vec<TypedValue> = Vec::new();
        let mut col_length: Vec<u16> = Vec::new();

        let mut offset = 0;

        for index in 0..column_map.ord_map().len() {
            // Extract the column length
            let col_len_bytes = &bytes[offset..offset + 2];
            let col_len = u16::from_be_bytes([col_len_bytes[0], col_len_bytes[1]]);
            offset += 2;
            // Store the column length
            col_length.push(col_len);
        }

        for (index, len) in col_length.iter().enumerate() {
            if *len == 0 {
                columns.push(TypedValue::NULL(NULL::new()));
            } else {
                let col_name = &column_map.ord_map()[index];
                let column_entry = column_map.get_column(col_name).unwrap();
                let typed_value =
                    TypedValue::from_bytes(&bytes[offset..offset + *len as usize], column_entry);
                columns.push(typed_value);
                offset += *len as usize;
            }
        }
        Self {
            col_length,
            columns,
        }
    }

    pub fn col_len(&self) -> &Vec<u16> {
        &self.col_length
    }

    pub fn columns(&self) -> &Vec<TypedValue> {
        &self.columns
    }
}
