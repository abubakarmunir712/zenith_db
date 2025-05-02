/// Represents a TINYINT (8-bit signed integer) data type.
use serde::{Deserialize, Serialize};

use crate::enums::type_errors::TypeError;

#[derive(Serialize, Deserialize, Debug)]
pub struct TINYINT {
    value: i8,
}

impl TINYINT {
    /// Creates a new TINYINT instance.
    pub fn new(value: &str) -> Result<Self, String> {
        let value: i8 = value
            .parse()
            .map_err(|e| TypeError::MismatchedDataType.message(value, "TINYINT"))?;
        Ok(TINYINT { value })
    }

    /// Converts the given i8 value to a 1-byte little-endian representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    /// Converts a 1-byte little-endian representation back to an i8 value.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 1] = bytes.try_into().unwrap();
        TINYINT {
            value: i8::from_le_bytes(bytes),
        }
    }

    pub fn value(&self)->i8{
        self.value
    }
}
