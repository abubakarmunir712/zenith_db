use crate::enums::datatypes::DataType;
use crate::storage::catalog::CatalogTable;
use crate::storage::page::Page;
use crate::storage::record::Record;
use crate::storage::catalog::ColumnInfo;

pub struct RecordManager;

impl RecordManager {
    pub fn to_human_readable(
        page: &Page,
        catalog: &CatalogTable,
        record_start: usize,
    ) -> Vec<DataType> {
        let buffer = &page.data;
        let record = Record::deserialize(buffer, record_start);

        let mut result: Vec<DataType> = Vec::new();
        let mut fixed_offset = 0;
        let mut variable_offset = 0;
        let mut var_size_index = 0;

        // Handle Fixed Columns
        for col in &catalog.fixed_columns {
            let size = col.size();
            let field_data = &record.fixed_fields_data[fixed_offset..fixed_offset + size];
            let dt = DataType::from_bytes(field_data, col).expect("Failed to convert fixed field");
            result.push(dt);
            fixed_offset += size;
        }

        // Handle Variable Columns
        for col in &catalog.variable_columns {
            let size = record.sizes_of_variable_fields[var_size_index] as usize;
            let field_data = &record.variable_fields_data[variable_offset..variable_offset + size];
            let dt = DataType::from_bytes(field_data, col).expect("Failed to convert variable field");
            result.push(dt);
            variable_offset += size;
            var_size_index += 1;
        }

        result
    }

    pub fn from_human_readable(
        page: &mut Page,
        catalog: &CatalogTable,
        record_start: usize,
        attributes: Vec<DataType>,
    ) {
        let mut record = Record {
            total_size_of_fixed_fields: 0,
            no_of_variable_fields: 0,
            sizes_of_variable_fields: Vec::new(),
            fixed_fields_data: Vec::new(),
            variable_fields_data: Vec::new(),
        };

        let mut attr_index = 0;

        // Fixed columns
        for col in &catalog.fixed_columns {
            let attr = &attributes[attr_index];
            let bytes = attr.to_bytes(col).expect("Failed to convert fixed attribute");
            record.fixed_fields_data.extend(bytes);
            attr_index += 1;
        }

        record.total_size_of_fixed_fields = record.fixed_fields_data.len() as u16;

        // Variable columns
        for col in &catalog.variable_columns {
            let attr = &attributes[attr_index];
            let bytes = attr.to_bytes(col).expect("Failed to convert variable attribute");
            record.sizes_of_variable_fields.push(bytes.len() as u16);
            record.variable_fields_data.extend(bytes);
            attr_index += 1;
        }

        record.no_of_variable_fields = record.sizes_of_variable_fields.len() as u16;

        record.serialize(&mut page.data, record_start);
    }
}
