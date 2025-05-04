use crate::{
    configs::db_internal_configs::DbConfigs::MAX_TABLE_NAME_LENGTH,
    enums::catlog_errors::CatalogError,
};

pub struct TableEntry {
    table_name: String,
    oid: u16,
    columns: u16,
    col_map_pg_num: u32,
    no_of_cols_in_primary_key: u8,
}

impl TableEntry {
    pub fn new(
        table_name: String,
        oid: u16,
        columns: u16,
        col_map_pg_num: u32,
        no_of_cols_in_primary_key: u8,
    ) -> Result<Self, String> {
        if table_name.len() > MAX_TABLE_NAME_LENGTH {
            return Err(CatalogError::TableNameLengthLimitExceeded
                .message()
                .to_string());
        }
        Ok(Self {
            table_name,
            oid,
            columns,
            col_map_pg_num,
            no_of_cols_in_primary_key,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(72);

        // Serialize table_name
        let name_bytes = self.table_name.as_bytes();
        let name_len = name_bytes.len();

        bytes.push(name_len as u8); // First byte = length
        bytes.extend_from_slice(&name_bytes[..name_len]);

        // Pad remaining to make it 63 bytes total
        for _ in name_len..63 {
            bytes.push(0);
        }

        // Serialize oid (2 bytes)
        bytes.extend_from_slice(&self.oid.to_le_bytes());

        // Serialize columns (2 bytes)
        bytes.extend_from_slice(&self.columns.to_le_bytes());

        // Serialize col_map_pg_num (4 bytes)
        bytes.extend_from_slice(&self.col_map_pg_num.to_le_bytes());

        // Serialize no_of_cols_in_primary_key (1 byte)
        bytes.push(self.no_of_cols_in_primary_key);

        bytes
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let name_len = data[0] as usize;
        let name_slice = &data[1..1 + name_len];
        let table_name = String::from_utf8_lossy(name_slice).to_string();

        let oid = u16::from_le_bytes([data[64], data[65]]);
        let columns = u16::from_le_bytes([data[66], data[67]]);
        let col_map_pg_num = u32::from_le_bytes([data[68], data[69], data[70], data[71]]);
        let no_of_cols_in_primary_key = data[72];

        Self {
            table_name,
            oid,
            columns,
            col_map_pg_num,
            no_of_cols_in_primary_key,
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

    pub fn col_map_pg_num(&self) -> u32 {
        self.col_map_pg_num
    }

    pub fn no_of_cols_in_primary_key(&self) -> u8 {
        self.no_of_cols_in_primary_key
    }

    // Setters
    pub fn set_table_name(&mut self, name: String) {
        self.table_name = name;
    }

    pub fn set_oid(&mut self, oid: u16) {
        self.oid = oid;
    }

    pub fn set_columns(&mut self, cols: u16) {
        self.columns = cols;
    }

    pub fn set_col_map_pg_num(&mut self, page_num: u32) {
        self.col_map_pg_num = page_num;
    }

    pub fn set_no_of_cols_in_primary_key(&mut self, count: u8) {
        self.no_of_cols_in_primary_key = count;
    }
}
