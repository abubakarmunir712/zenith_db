use crate::enums::{catlog_errors::CatalogError, datatypes::DataType};

pub struct ColumnEntry {
    pub column_name: String,
    pub oid: u16,
    pub datatype: DataType,
    pub max_size: u32,
    pub null: bool,
    pub unique: bool,
    pub is_primary_key: bool,
    pub is_foreign_key: bool,
    pub is_referenced: bool,
}

impl ColumnEntry {
    pub fn new(
        column_name: String,
        oid: u16,
        datatype: DataType,
        max_size: u32,
        null: bool,
        unique: bool,
        is_primary_key: bool,
        is_foreign_key: bool,
        is_referenced: bool,
    ) -> Result<Self, String> {
        if column_name.len() > 63 {
            return Err(CatalogError::ColumnNameLengthLimitExceeded
                .message()
                .to_string());
        }

        Ok(Self {
            column_name,
            oid,
            datatype,
            max_size,
            null,
            unique,
            is_primary_key,
            is_foreign_key,
            is_referenced,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(80);

        // Column name
        let name_bytes = self.column_name.as_bytes();
        let name_len = name_bytes.len();
        bytes.push(name_len as u8);
        bytes.extend_from_slice(name_bytes);
        for _ in name_len..63 {
            bytes.push(0);
        }

        // OID
        bytes.extend_from_slice(&self.oid.to_le_bytes());

        // DataType as u8
        bytes.push(self.datatype.to_oid());

        // Max size
        bytes.extend_from_slice(&self.max_size.to_le_bytes());

        // Booleans as 1 byte each
        bytes.push(self.null as u8);
        bytes.push(self.unique as u8);
        bytes.push(self.is_primary_key as u8);
        bytes.push(self.is_foreign_key as u8);
        bytes.push(self.is_referenced as u8);

        bytes
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let name_len = data[0] as usize;
        let name_slice = &data[1..1 + name_len];
        let column_name = String::from_utf8_lossy(name_slice).to_string();

        let oid = u16::from_le_bytes([data[64], data[65]]);
        let datatype = DataType::from_oid(data[66]);
        let max_size = u32::from_le_bytes([data[67], data[68], data[69], data[70]]);

        let null = data[71] != 0;
        let unique = data[72] != 0;
        let is_primary_key = data[73] != 0;
        let is_foreign_key = data[74] != 0;
        let is_referenced = data[75] != 0;

        Self {
            column_name,
            oid,
            datatype,
            max_size,
            null,
            unique,
            is_primary_key,
            is_foreign_key,
            is_referenced,
        }
    }

}
