use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordHeader {
    schema: Vec<u8>, // Schema data
    record_length: usize, // Length of the entire record
    timestamp: u64, // Timestamp of creation
    field_pointers: Vec<Option<usize>>, // Index positions of variable-length fields (None for NULL values)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableRecord {
    header: RecordHeader,
    fixed_fields: Vec<i32>, // Fixed-size fields (e.g., integers)
    variable_fields: Vec<Option<String>>, // Variable-length fields (e.g., strings)
}

impl VariableRecord {
    pub fn new(schema: Vec<u8>, fixed_fields: Vec<i32>, variable_fields: Vec<Option<String>>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut record_length = fixed_fields.len() * std::mem::size_of::<i32>();
        let mut field_pointers = Vec::new();
        let mut offset = record_length;

        for field in &variable_fields {
            if let Some(data) = field {
                field_pointers.push(Some(offset));
                offset += data.len();
            } else {
                field_pointers.push(None);
            }
        }
        record_length = offset;

        let header = RecordHeader {
            schema,
            record_length,
            timestamp,
            field_pointers,
        };

        Self {
            header,
            fixed_fields,
            variable_fields,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Failed to serialize record")
    }

    pub fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).expect("Failed to deserialize record")
    }
}

fn main() {
    let schema = vec![0; 10];
    let fixed_fields = vec![100, 200, 300, 400];
    let variable_fields = vec![Some("Alice".to_string()), None, Some("Bob".to_string())];

    let record = VariableRecord::new(schema, fixed_fields, variable_fields);
    let serialized = record.serialize();
    let deserialized: VariableRecord = VariableRecord::deserialize(&serialized);
    
    println!("Original Record: {:?}", record);
    println!("Deserialized Record: {:?}", deserialized);
}
