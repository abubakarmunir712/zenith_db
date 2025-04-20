use crate::configs::config::Config::PAGE_SIZE;

/// A `Record` represents a single data unit consisting of fixed and variable size fields.
/// 
/// The structure supports serialization to and deserialization from a raw byte buffer.
/// It uses little-endian byte order for encoding numeric fields.
pub struct Record {
    /// Total size (in bytes) of all fixed-length fields.
    pub total_size_of_fixed_fields: u16,

    /// Number of variable-length fields present in the record.
    pub no_of_variable_fields: u16,

    /// A list containing the sizes (in bytes) of each variable-length field.
    pub sizes_of_variable_fields: Vec<u16>,

    /// Raw bytes representing all fixed-length fields.
    pub fixed_fields_data: Vec<u8>,

    /// Raw bytes representing all variable-length fields concatenated together.
    pub variable_fields_data: Vec<u8>,
}

impl Record {
    /// Serializes the `Record` into the provided buffer starting from `starting_offset`.
    /// 
    /// # Arguments
    /// * `buffer` - A mutable reference to a byte vector where the serialized data will be stored.
    /// * `starting_offset` - The index in the buffer where serialization should begin.
    /// 
    /// # Details
    /// This method writes the fixed fields first, followed by the sizes of variable fields,
    /// then the fixed fields data, and finally the variable fields data.
    pub fn serialize(&self, buffer: &mut Vec<u8>, starting_offset: usize) {
        let mut offset = starting_offset;

        // Write total size of fixed fields (2 bytes)
        buffer[offset..offset + 2].copy_from_slice(&self.total_size_of_fixed_fields.to_le_bytes());
        offset += 2;

        // Write number of variable fields (2 bytes)
        buffer[offset..offset + 2].copy_from_slice(&self.no_of_variable_fields.to_le_bytes());
        offset += 2;

        // Write sizes of each variable field (2 bytes each)
        for size in &self.sizes_of_variable_fields {
            buffer[offset..offset + 2].copy_from_slice(&size.to_le_bytes());
            offset += 2;
        }

        // Write all fixed fields data
        buffer[offset..offset + self.fixed_fields_data.len()]
            .copy_from_slice(&self.fixed_fields_data);
        offset += self.fixed_fields_data.len();

        // Write all variable fields data
        buffer[offset..offset + self.variable_fields_data.len()]
            .copy_from_slice(&self.variable_fields_data);
    }

    /// Deserializes a `Record` from the given byte buffer starting from `starting_offset`.
    /// 
    /// # Arguments
    /// * `buffer` - A reference to the byte vector containing the serialized record data.
    /// * `starting_offset` - The index in the buffer where deserialization should begin.
    /// 
    /// # Returns
    /// A `Record` instance populated with the extracted data.
    /// 
    /// # Details
    /// This method reads the data in the same order it was written during serialization.
    pub fn deserialize(buffer: &Vec<u8>, starting_offset: usize) -> Self {
        let mut offset = starting_offset;
    
        // Read total size of fixed fields (2 bytes)
        let total_size_of_fixed_fields =
            u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
        offset += 2;
    
        // Read number of variable fields (2 bytes)
        let no_of_variable_fields =
            u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
        offset += 2;
    
        // (Deserialization continues...)
        let mut sizes_of_variable_fields = Vec::with_capacity(no_of_variable_fields as usize);

        // Read sizes of variable fields
        for _ in 0..no_of_variable_fields {
            let size = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
            sizes_of_variable_fields.push(size);
            offset += 2;
        }

        // Read fixed fields data
        let fixed_fields_data = buffer[offset..offset + total_size_of_fixed_fields as usize].to_vec();
        offset += total_size_of_fixed_fields as usize;

        // Calculate total size of variable fields
        let total_variable_data_size: usize = sizes_of_variable_fields.iter().map(|&s| s as usize).sum();

        // Read variable fields data
        let variable_fields_data = buffer[offset..offset + total_variable_data_size].to_vec();
    
        Record {
            total_size_of_fixed_fields,
            no_of_variable_fields,
            sizes_of_variable_fields,
            fixed_fields_data,
            variable_fields_data,
        }
    }

    pub fn total_record_size_in_bytes(&self) -> u16
    {
        let size_of_header = 2 + 2; // total_size_of_fixed_fields + no_of_variable_fields
        let size_of_variable_sizes = self.sizes_of_variable_fields.len() * 2;
        let size_of_fixed_data = self.fixed_fields_data.len();
        let size_of_variable_data = self.variable_fields_data.len();

        (size_of_header + size_of_variable_sizes + size_of_fixed_data + size_of_variable_data) as u16
    }
}
