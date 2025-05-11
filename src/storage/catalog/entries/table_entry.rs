use crate::{
    configs::db_internal_configs::DbConfigs::MAX_TABLE_NAME_LENGTH,
    enums::errors::catalog_errors::CatalogError,
};

pub struct TableEntry {
    table_name: String,
    oid: u16,
    columns: u16,
    no_of_cols_in_primary_key: u8,
    no_of_foregin_key_constraints: u8,
}

impl TableEntry {
    pub fn new(table_name: String, oid: u16) -> Result<Self, String> {
        if table_name.len() > MAX_TABLE_NAME_LENGTH {
            return Err(CatalogError::TableNameLengthLimitExceeded
                .message()
                .to_string());
        }
        Ok(Self {
            table_name,
            oid,
            columns: 0,
            no_of_cols_in_primary_key: 0,
            no_of_foregin_key_constraints: 0,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(70);

        // table_name (max 63 bytes)
        let name_bytes = self.table_name.as_bytes();
        let name_len = name_bytes.len();
        bytes.push(name_len as u8);
        bytes.extend_from_slice(&name_bytes[..name_len]);
        for _ in name_len..63 {
            bytes.push(0);
        }

        // oid (2 bytes)
        bytes.extend_from_slice(&self.oid.to_le_bytes());

        // columns (2 bytes)
        bytes.extend_from_slice(&self.columns.to_le_bytes());

        // no_of_cols_in_primary_key (1 byte)
        bytes.push(self.no_of_cols_in_primary_key);

        // no_of_foregin_key_constraints (1 byte)
        bytes.push(self.no_of_foregin_key_constraints);

        bytes
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let name_len = data[0] as usize;
        let name_slice = &data[1..1 + name_len];
        let table_name = String::from_utf8_lossy(name_slice).to_string();

        let oid = u16::from_le_bytes([data[64], data[65]]);
        let columns = u16::from_le_bytes([data[66], data[67]]);
        let no_of_cols_in_primary_key = data[68];
        let no_of_foregin_key_constraints = data[69];

        Self {
            table_name,
            oid,
            columns,
            no_of_cols_in_primary_key,
            no_of_foregin_key_constraints,
        }
    }

    // Getters
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn oid(&self) -> u16 {
        self.oid
    }

    pub fn columns(&self) -> u16 {
     
     
        self.columns
    }

    pub fn no_of_cols_in_primary_key(&self) -> u8 {
        self.no_of_cols_in_primary_key
    }

    pub fn no_of_foregin_key_constraints(&self) -> u8 {
        self.no_of_cols_in_primary_key
    }
    // Setters
    pub fn set_table_name(&mut self, name: String) {
        self.table_name = name;
    }

    pub fn set_oid(&mut self, oid: u16) {
        self.oid = oid;
    }

    pub fn increase_columns(&mut self) {
        self.columns +=1;
    }

    pub fn decrease_columns(&mut self) {
        self.columns -=1;
    }

    pub fn increase_no_of_cols_in_pk(&mut self) {
        self.no_of_cols_in_primary_key +=1;
    }

    pub fn decrease_no_of_cols_in_pk(&mut self) {
        self.no_of_cols_in_primary_key -=1;
    }
}

