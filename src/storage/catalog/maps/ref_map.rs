use super::super::entries::ref_entry::RefEntry;
use crate::configs::db_internal_configs::DbConfigs::MAX_REF_SIZE;
use std::collections::HashMap;

pub struct RefMap {
    no_of_ref: u16,
    map: Vec<RefEntry>,
}
