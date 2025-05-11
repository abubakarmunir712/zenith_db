use crate::{
    configs::db_internal_configs::DbConfigs::MAX_COLUMNS_IN_FOREIGN_KEY,
    enums::{errors::catalog_errors::CatalogError, types::cascading_type::ForeignKeyAction},
};

// Future Improvement:
// The f_table and r_table used for every pair in an entry will always be the same.
// Instead of repeating that info, store it once in `RefEntry`.
// This will reduce redundancy and make the schema cleaner.
pub struct RefPair {
    f_table_oid: Option<u16>,
    f_column_oid: Option<u16>,
    r_table_oid: Option<u16>,
    r_column_oid: Option<u16>,
}

impl RefPair {
    pub fn new(
        f_table_oid: Option<u16>,
        f_column_oid: Option<u16>,
        r_table_oid: Option<u16>,
        r_column_oid: Option<u16>,
    ) -> Self {
        Self {
            f_table_oid,
            f_column_oid,
            r_table_oid,
            r_column_oid,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        for opt in [
            self.f_table_oid,
            self.f_column_oid,
            self.r_table_oid,
            self.r_column_oid,
        ] {
            let val = opt.unwrap_or(0);
            bytes.extend_from_slice(&val.to_le_bytes());
        }
        bytes
    }

    pub fn deserialize(data: &[u8]) -> Self {
        fn read_opt(slice: &[u8]) -> Option<u16> {
            let val = u16::from_le_bytes([slice[0], slice[1]]);
            if val == 0 { None } else { Some(val) }
        }

        Self {
            f_table_oid: read_opt(&data[0..2]),
            f_column_oid: read_opt(&data[2..4]),
            r_table_oid: read_opt(&data[4..6]),
            r_column_oid: read_opt(&data[6..8]),
        }
    }

    // === Getters ===
    pub fn f_table_oid(&self) -> Option<u16> {
        self.f_table_oid
    }

    pub fn f_column_oid(&self) -> Option<u16> {
        self.f_column_oid
    }

    pub fn r_table_oid(&self) -> Option<u16> {
        self.r_table_oid
    }

    pub fn r_column_oid(&self) -> Option<u16> {
        self.r_column_oid
    }

    // === Setters ===
    pub fn set_f_table_oid(&mut self, oid: Option<u16>) {
        self.f_table_oid = oid;
    }

    pub fn set_f_column_oid(&mut self, oid: Option<u16>) {
        self.f_column_oid = oid;
    }

    pub fn set_r_table_oid(&mut self, oid: Option<u16>) {
        self.r_table_oid = oid;
    }

    pub fn set_r_column_oid(&mut self, oid: Option<u16>) {
        self.r_column_oid = oid;
    }
}

pub struct RefEntry {
    reference: Vec<RefPair>,
    cascading_type: ForeignKeyAction,
}

impl RefEntry {
    pub fn new(reference: Vec<RefPair>, cascading_type: ForeignKeyAction) -> Result<Self, String> {
        if reference.len() > MAX_COLUMNS_IN_FOREIGN_KEY {
            return Err(CatalogError::SysMaxColumnsInFKExceeded
                .message()
                .to_string());
        }
        Ok(Self {
            reference,
            cascading_type,
        })
    }
}

impl RefEntry {
    // === Serialize (fixed 32 bytes) ===
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; 32];

        // First byte: number of ref pairs
        let count = self.reference.len().min(MAX_COLUMNS_IN_FOREIGN_KEY);
        bytes[0] = count as u8;

        // Second byte: cascading type as u8
        bytes[1] = self.cascading_type.to_oid();

        // Each pair takes 8 bytes: 4 * u16
        for (i, pair) in self.reference.iter().take(3).enumerate() {
            let pair_bytes = pair.serialize(); // 8 bytes
            let start = 2 + (i * 8);
            let end = start + 8;
            bytes[start..end].copy_from_slice(&pair_bytes);
        }

        bytes
    }

    // === Deserialize (expects 32 bytes) ===
    pub fn deserialize(data: &[u8]) -> Self {
        let count = data[0] as usize;
        let action = ForeignKeyAction::from_oid(data[1]); // You need to implement this

        let mut reference = Vec::with_capacity(count);
        for i in 0..count.min(3) {
            let start = 2 + (i * 8);
            let end = start + 8;
            reference.push(RefPair::deserialize(&data[start..end]));
        }

        Self {
            reference,
            cascading_type: action,
        }
    }

    // === Getters ===
    pub fn references(&self) -> &Vec<RefPair> {
        &self.reference
    }

    pub fn cascading_type(&self) -> &ForeignKeyAction {
        &self.cascading_type
    }

    // === Setters ===
    pub fn set_references(&mut self, refs: Vec<RefPair>) {
        self.reference = refs;
    }

    pub fn set_cascading_type(&mut self, action: ForeignKeyAction) {
        self.cascading_type = action;
    }
}
