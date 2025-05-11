use crate::storage::catalog::maps::{column_map::ColumnMap, ref_map::RefMap, table_map::TableMap};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum CatalogType {
    TableMap,
    ColumnMap,
    RefMap,
}

pub enum CatalogData {
    TableMap(TableMap),
    ColumnMap(ColumnMap),
    RefMap(RefMap),
}

impl CatalogData {
    pub fn serialize(&self, buffer: &mut [u8]) {
        match &self {
            CatalogData::ColumnMap(d) => d.serialize(buffer),
            CatalogData::TableMap(d) => d.serialize(buffer),
            CatalogData::RefMap(d) => d.serialize(buffer),
        };
    }

    pub fn deserialize(buffer: &[u8], catalog_type: CatalogType) -> Self {
        match catalog_type {
            CatalogType::ColumnMap => CatalogData::ColumnMap(ColumnMap::deserialize(buffer)),
            CatalogType::TableMap => CatalogData::TableMap(TableMap::deserialize(buffer)),
            CatalogType::RefMap => CatalogData::RefMap(RefMap::deserialize(buffer)),
        }
    }
}
