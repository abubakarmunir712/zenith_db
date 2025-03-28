use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

/// Represents a database record with fixed and variable-length fields.
pub struct Record {
    schema_ptr: u32,                // Pointer to schema
    record_length: u16,             // Total length of the record (bytes)
    timestamp: u32,                 // Creation/modification timestamp
    var_field_pointers: Vec<Option<u32>>, // Offsets for variable-length fields
    fixed_fields: Vec<u8>,          // Fixed-size fields stored in order
    variable_fields: Vec<Vec<u8>>,  // Actual variable-length field data
}

impl Record {
    /// Creates a new record with given fixed and variable fields.
    pub fn new(schema_ptr: u32, fixed_fields: Vec<u8>, variable_fields: Vec<Option<Vec<u8>>>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32;

        let mut record_length = 4 + 2 + 4;
        record_length += (4 * variable_fields.len()) as u16;

        let mut var_field_pointers = Vec::new();
        let mut stored_variable_fields = Vec::new();
        let mut offset = fixed_fields.len() as u32;

        for field in variable_fields {
            match field {
                Some(data) => {
                    var_field_pointers.push(Some(offset));
                    offset += data.len() as u32;
                    stored_variable_fields.push(data);
                }
                None => var_field_pointers.push(None),
            }
        }

        record_length += fixed_fields.len() as u16;
        record_length += offset as u16 - fixed_fields.len() as u16;

        Self {
            schema_ptr,
            record_length,
            timestamp,
            var_field_pointers,
            fixed_fields,
            variable_fields: stored_variable_fields,
        }
    }

    /// Serializes the record into a byte array.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self._serialize_metadata(&mut buffer);
        self._serialize_data(&mut buffer);
        buffer
    }

    fn _serialize_metadata(&self, buffer: &mut Vec<u8>) {
        buffer.extend(&self.schema_ptr.to_le_bytes());
        buffer.extend(&self.record_length.to_le_bytes());
        buffer.extend(&self.timestamp.to_le_bytes());
        for &ptr in &self.var_field_pointers {
            match ptr {
                Some(offset) => buffer.extend(&offset.to_le_bytes()),
                None => buffer.extend(&0u32.to_le_bytes()),
            }
        }
    }

    fn _serialize_data(&self, buffer: &mut Vec<u8>) {
        buffer.extend(&self.fixed_fields);
        for field in &self.variable_fields {
            buffer.extend(field);
        }
    }

    /// Deserializes a byte slice into a Record object.
    pub fn deserialize(data: &[u8], var_field_count: usize) -> Self {
        let schema_ptr = u32::from_le_bytes(data[0..4].try_into().expect("Invalid schema_ptr"));
        let record_length = u16::from_le_bytes(data[4..6].try_into().expect("Invalid record_length"));
        let timestamp = u32::from_le_bytes(data[6..10].try_into().expect("Invalid timestamp"));
        
        let mut offset = 10;
        let mut var_field_pointers = Vec::new();
        
        for _ in 0..var_field_count {
            let pointer = u32::from_le_bytes(data[offset..offset + 4].try_into().expect("Invalid pointer"));
            offset += 4;
            var_field_pointers.push(if pointer == 0 { None } else { Some(pointer) });
        }
        
        let fixed_fields_end = offset;
        let fixed_fields = data.get(fixed_fields_end..(fixed_fields_end + (record_length as usize - offset)))
            .unwrap_or(&[]).to_vec();
        
        let mut variable_fields = Vec::new();
        for i in 0..var_field_pointers.len() {
            if let Some(pos) = var_field_pointers[i] {
                let start = pos as usize;
                let end = if i + 1 < var_field_pointers.len() {
                    var_field_pointers[i + 1].unwrap_or(record_length as u32) as usize
                } else {
                    data.len()
                };
                if start < end && end <= data.len() {
                    variable_fields.push(data[start..end].to_vec());
                } else {
                    variable_fields.push(vec![]);
                }
            } else {
                variable_fields.push(vec![]);
            }
        }
        
        Self {
            schema_ptr,
            record_length,
            timestamp,
            var_field_pointers,
            fixed_fields,
            variable_fields,
        }
    }
}

fn main() {
    let fixed_fields = vec![1, 2, 3, 4, 5];
    let variable_fields = vec![
        Some(vec![10, 20, 30]),
        None,
        Some(vec![40, 50, 60, 70]),
    ];
    
    let record = Record::new(123456, fixed_fields.clone(), variable_fields.clone());
    let serialized = record.serialize();
    println!("Serialized Record: {:?}", serialized);
    
    let deserialized = Record::deserialize(&serialized, variable_fields.len());
    println!("Expected Fixed Fields: {:?}", fixed_fields);
    println!("Deserialized Fixed Fields: {:?}", deserialized.fixed_fields);
    println!("Expected Variable Fields: {:?}", variable_fields);
    println!("Deserialized Variable Fields: {:?}", deserialized.variable_fields);
}