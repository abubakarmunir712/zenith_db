/// Represents a BIGINT (64-bit signed integer) data type.
use serde::{Deserialize, Serialize};

use crate::enums::type_errors::TypeError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BIGINT {
    pub value: i64,
}

impl BIGINT {
    /// Creates a new BIGINT instance.
    pub fn new(value: &str) -> Result<Self, String> {
        let value: i64 = value
            .parse()
            .map_err(|e| TypeError::MismatchedDataType.message(value, "BIGINT"))?;
        Ok(BIGINT { value })
    }

    /// Converts the given i64 value to an 8-byte little-endian representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    /// Converts an 8-byte little-endian representation back to an i64 value.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 8] = bytes.try_into().unwrap();
        BIGINT {
            value: i64::from_le_bytes(bytes),
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}
