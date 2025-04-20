/// Represents a BIGINT (64-bit signed integer) data type.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct BIGINT {
    pub value: i64,
}

impl BIGINT {
    /// Creates a new BIGINT instance.
    pub fn new(value: i64) -> Self {
        BIGINT { value }
    }

    /// Converts the given i64 value to an 8-byte little-endian representation.
    pub fn to_bytes(&self) -> [u8; 8] {
        self.value.to_le_bytes()
    }

    /// Converts an 8-byte little-endian representation back to an i64 value.
    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        BIGINT {
            value: i64::from_le_bytes(*bytes),
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}
