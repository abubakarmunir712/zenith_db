use super::super::entries::ref_entry::RefEntry;
use crate::{
    configs::{config::Config::REF_PAGE_SIZE, db_internal_configs::DbConfigs::MAX_REF_SIZE},
    enums::catalog_errors::CatalogError,
};

pub struct RefMap {
    no_of_ref: u16,
    map: Vec<RefEntry>,
}

impl RefMap {
    pub fn new() -> Self {
        Self {
            no_of_ref: 0,
            map: Vec::new(),
        }
    }

    pub fn insert_ref(&mut self, ref_entry: RefEntry) -> Result<(), String> {
        if self.no_of_ref as usize >= MAX_REF_SIZE {
            return Err(CatalogError::SysMaxRefPerPageLimitExceeded
                .message()
                .to_string());
        }
        self.map.push(ref_entry);
        self.no_of_ref += 1;
        Ok(())
    }

    pub fn serialize(&self, buffer: &mut [u8]) {
        // First 2 bytes = number of references
        buffer[0..2].copy_from_slice(&self.no_of_ref.to_le_bytes());

        for (i, entry) in self.map.iter().enumerate() {
            let entry_bytes = entry.serialize();
            let offset = 2 + (i * 32);
            buffer[offset..offset + 32].copy_from_slice(&entry_bytes);
        }
    }

    /// Deserializes a RefMap from a &[u8]
    pub fn deserialize(data: &[u8]) -> Self {
        let no_of_ref = u16::from_le_bytes([data[0], data[1]]);
        let mut map = Vec::with_capacity(no_of_ref as usize);

        for i in 0..no_of_ref {
            let start = 2 + (i as usize * 32);
            let end = start + 32;
            let ref_entry = RefEntry::deserialize(&data[start..end]);
            map.push(ref_entry);
        }

        Self { no_of_ref, map }
    }

    pub fn no_of_ref(&self) -> u16 {
        self.no_of_ref
    }

    pub fn map(&self) -> &Vec<RefEntry> {
        &self.map
    }
}
