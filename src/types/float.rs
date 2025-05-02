/// Represents a FLOAT (32-bit floating point) data type.
use serde::{Deserialize, Serialize};

use crate::enums::type_errors::TypeError;

#[derive(Serialize, Deserialize, Debug)]
pub struct FLOAT {
    value: f32,
}

impl FLOAT {
    /// Creates a new FLOAT instance.
    pub fn new(value: &str) -> Result<Self, String> {
        let value: f32 = value
            .parse()
            .map_err(|e| TypeError::MismatchedDataType.message(value, "FLOAT"))?;
        Ok(FLOAT { value })
    }

    /// Converts the FLOAT value to a 4-byte little-endian representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    /// Converts a 4-byte little-endian representation back to a FLOAT value.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 4] = bytes.try_into().unwrap();
        FLOAT {
            value: f32::from_le_bytes(bytes),
        }
    }
}
