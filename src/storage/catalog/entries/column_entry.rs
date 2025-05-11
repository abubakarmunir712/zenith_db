use crate::enums::{errors::catalog_errors::CatalogError, types::datatypes::DataType};

pub struct ColumnEntry {
    column_name: String,
    oid: u16,
    datatype: DataType,
    max_size: u32,
    null: bool,
    unique: bool,
    is_primary_key: bool,
    is_foreign_key: bool,
    reference_count: u8,
}

impl ColumnEntry {
    pub fn new(
        column_name: String,
        oid: u16,
        datatype: DataType,
        max_size: u32,
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
            null: false,
            unique: false,
            is_primary_key: false,
            is_foreign_key: false,
            reference_count: 0,
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
        bytes.push(self.reference_count as u8);

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
        let reference_count = data[75];

        Self {
            column_name,
            oid,
            datatype,
            max_size,
            null,
            unique,
            is_primary_key,
            is_foreign_key,
            reference_count,
        }
    }

    // === Getters ===

    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    pub fn oid(&self) -> u16 {
        self.oid
    }

    pub fn datatype(&self) -> &DataType {
        &self.datatype
    }

    pub fn max_size(&self) -> u32 {
        self.max_size
    }

    pub fn is_nullable(&self) -> bool {
        self.null
    }

    pub fn is_unique(&self) -> bool {
        self.unique
    }

    pub fn is_primary_key(&self) -> bool {
        self.is_primary_key
    }

    pub fn is_foreign_key(&self) -> bool {
        self.is_foreign_key
    }

    pub fn reference_count(&self) -> u8 {
        self.reference_count
    }

    // === Setters / Mutators ===

    pub fn set_name(&mut self, name: String) {
        self.column_name = name;
    }

    pub fn set_oid(&mut self, oid: u16) {
        self.oid = oid;
    }

    pub fn set_datatype(&mut self, dtype: DataType) {
        self.datatype = dtype;
    }

    pub fn set_max_size(&mut self, size: u32) {
        self.max_size = size;
    }

    pub fn make_nullable(&mut self) {
        self.null = true;
    }

    pub fn make_not_nullable(&mut self) {
        self.null = false;
    }

    pub fn make_unique(&mut self) {
        self.unique = true;
    }

    pub fn remove_unique(&mut self) {
        self.unique = false;
    }

    pub fn make_primary(&mut self) {
        self.is_primary_key = true;
    }

    pub fn make_foreign(&mut self) {
        self.is_foreign_key = true;
    }

    pub fn remove_primary(&mut self) {
        self.is_primary_key = false;
    }

    pub fn remove_foreign(&mut self) {
        self.is_foreign_key = false;
    }

    pub fn increase_reference_count(&mut self) {
        self.reference_count += 1;
    }

    pub fn decrease_reference_count(&mut self) {
        if self.reference_count > 0 {
            self.reference_count -= 1;
        }
    }
}
