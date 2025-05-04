use crate::{
    configs::db_internal_configs::DbConfigs::MAX_COLUMNS_IN_FOREIGN_KEY,
    enums::{cascading_type::ForeignKeyAction, catlog_errors::CatalogError},
};

pub struct RefEntry {
    reference: Vec<RefPair>,
    cascading_type: ForeignKeyAction,
}

pub struct RefPair {
    f_table_oid: Option<u16>,
    f_column_oid: Option<u16>,
    r_table_oid: Option<u16>,
    r_column_oid: Option<u16>,
}

impl RefEntry {
    pub fn new_refernce(
        reference: Vec<RefPair>,
        cascading_type: ForeignKeyAction,
    ) -> Result<Self, String> {
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
