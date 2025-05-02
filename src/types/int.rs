/// Represents an INT (32-bit signed integer) data type.
use serde::{Deserialize, Serialize};

use crate::enums::type_errors::TypeError;

#[derive(Serialize, Deserialize, Debug)]
pub struct INT {
    value: i32,
}

impl INT {
    /// Creates a new INT instance.
    pub fn new(value: &str) -> Result<Self, String> {
        let value: i32 = value
            .parse()
            .map_err(|e| TypeError::MismatchedDataType.message(value, "INT"))?;
        Ok(INT { value })
    }

    /// Converts the given i32 value to a 4-byte little-endian representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    /// Converts a 4-byte little-endian representation back to an i32 value.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 4] = bytes.try_into().unwrap();
        INT {
            value: i32::from_le_bytes(bytes),
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
