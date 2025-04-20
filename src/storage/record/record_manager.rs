use crate::enums::datatypes::DataType;
use crate::storage::catalog::catalog::CatalogTable;
use crate::storage::page::page::Page;
use crate::storage::record::record::Record;

/// Struct representing the Record Manager, responsible for converting between
/// raw bytes and human-readable `DataType` representations.
pub struct RecordManager;

impl RecordManager {
    /// Converts a record in a page starting at a given offset into a list of `DataType` values.
    ///
    /// # Arguments
    /// * `page` - Reference to the `Page` containing raw data.
    /// * `catalog` - Reference to the `CatalogTable` describing the table schema.
    /// * `record_start` - Starting byte offset of the record in the page.
    ///
    /// # Returns
    /// * `Vec<DataType>` - List of deserialized fields in human-readable form.
    pub fn to_human_readable(
        page: &Page,
        catalog: &CatalogTable,
        record_start: usize,
    ) -> Vec<DataType> {
        let buffer = &page.data; // Raw bytes of the page
        let record = Record::deserialize(buffer, record_start); // Deserialize record from bytes

        let mut result: Vec<DataType> = Vec::new(); // Vector to hold deserialized fields
        let mut fixed_offset = 0; // Offset tracker for fixed-length fields
        let mut variable_offset = 0; // Offset tracker for variable-length fields
        let mut var_size_index = 0; // Index tracker for variable field sizes

        // Handle Fixed Columns
        for col in &catalog.fixed_columns {
            let size = col.size(); // Get size of fixed column
            let field_data = &record.fixed_fields_data[fixed_offset..fixed_offset + size]; // Slice field data
            let dt = DataType::from_bytes(field_data, col).expect("Failed to convert fixed field"); // Deserialize field data to DataType
            result.push(dt); // Add deserialized field to result
            fixed_offset += size; // Move to next field
        }

        // Handle Variable Columns
        for col in &catalog.variable_columns {
            let size = record.sizes_of_variable_fields[var_size_index] as usize; // Get stored size of variable field
            let field_data = &record.variable_fields_data[variable_offset..variable_offset + size]; // Slice field data
            let dt =
                DataType::from_bytes(field_data, col).expect("Failed to convert variable field"); // Deserialize field data to DataType
            result.push(dt); // Add deserialized field to result
            variable_offset += size; // Move to next field
            var_size_index += 1; // Move to next size
        }

        result // Return the deserialized list
    }

    /// Converts a list of `DataType` values into a serialized record and inserts it into a page.
    /// This function takes human-readable `DataType` attributes, converts them into their byte
    /// representations, and stores the resulting record into the given page.
    ///
    /// # Arguments
    /// * `page` - Mutable reference to the `Page` where the record will be written.
    /// * `catalog` - Reference to the `CatalogTable` describing the table schema.
    /// * `record_start` - Starting byte offset in the page where the record will be inserted.
    /// * `attributes` - A vector of `DataType` values representing the human-readable record.
    ///
    /// # Function Workflow
    /// 1. Initialize an empty `Record` structure to hold the serialized fields.
    /// 2. Process fixed columns: Convert each attribute to bytes and store it in the `fixed_fields_data` vector.
    /// 3. Update `total_size_of_fixed_fields` to reflect the total size of all fixed fields.
    /// 4. Process variable columns: Convert each attribute to bytes, update the `sizes_of_variable_fields` vector, and
    ///    store the variable-length field data.
    /// 5. Return Record
    pub fn from_human_readable(
        catalog: &CatalogTable,
        attributes: Vec<DataType>,
    ) -> Record{
        // Initialize an empty record to store the fixed and variable field data
        let mut record = Record {
            total_size_of_fixed_fields: 0,        // Total size of all fixed fields
            no_of_variable_fields: 0,             // Number of variable fields in the record
            sizes_of_variable_fields: Vec::new(), // Sizes of each variable field
            fixed_fields_data: Vec::new(),        // Data of fixed fields
            variable_fields_data: Vec::new(),     // Data of variable fields
        };

        let mut attr_index = 0; // Index to track current attribute being processed

        // Process Fixed Columns
        for col in &catalog.fixed_columns {
            // Get the corresponding attribute
            let attr = &attributes[attr_index];

            // Convert attribute to bytes for the fixed column and add to fixed fields
            let bytes = attr
                .to_bytes(col)
                .expect("Failed to convert fixed attribute");
            record.fixed_fields_data.extend(bytes); // Append bytes to the record

            attr_index += 1; // Move to the next attribute
        }

        // Set the total size of fixed fields
        record.total_size_of_fixed_fields = record.fixed_fields_data.len() as u16;

        // Process Variable Columns
        for col in &catalog.variable_columns {
            // Get the corresponding attribute
            let attr = &attributes[attr_index];

            // Convert attribute to bytes for the variable column
            let bytes = attr
                .to_bytes(col)
                .expect("Failed to convert variable attribute");

            // Store the size of the variable field
            record.sizes_of_variable_fields.push(bytes.len() as u16);

            // Append the bytes to the variable fields data
            record.variable_fields_data.extend(bytes);

            attr_index += 1; // Move to the next attribute
        }

        // Set the total number of variable fields
        record.no_of_variable_fields = record.sizes_of_variable_fields.len() as u16;
        record

        // // Serialize the record into the page at the specified location
        // record.serialize(&mut page.data, record_start);
    }

    pub fn insert_record(
        page: &mut Page,
        catalog: &CatalogTable,
        attributes: Vec<DataType>,
    ) {

        let record: Record= RecordManager::from_human_readable(catalog,attributes);
        let record_size: u16=record.total_record_size_in_bytes();

        let insert_offset=RecordManager::calculate_suitable_offset_for_record(record_size, page);
        record.serialize(&mut page.data, insert_offset as usize);
        page.insert_slot(insert_offset,record_size);

    }


    pub fn calculate_suitable_offset_for_record(record_size: u16, page: &Page) -> u16 {
        let free_space_table = page.free_space_table();
        let mut best_offset = 0;
        let mut best_size = u16::MAX;

        for (offset, size) in free_space_table {
            if size >= record_size && size < best_size {
                best_size = size;
                best_offset = offset;
            }
        }

        best_offset
    }
}
