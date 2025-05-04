use super::super::entries::ref_entry::RefEntry;
use crate::configs::db_internal_configs::DbConfigs::MAX_REF_SIZE;
use std::collections::HashMap;

pub struct RefMap {
    no_of_ref: u16,
    column_oid_bitmap: [u8; MAX_REF_SIZE / 8],
    map: HashMap<String, RefEntry>,
    next_ref_map: u32,
}
