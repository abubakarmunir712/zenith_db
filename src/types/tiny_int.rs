/// Represents a TINYINT (8-bit signed integer) data type.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct TINYINT {
    value: i8,
}

impl TINYINT {
    /// Creates a new TINYINT instance.
    pub fn new(value: i8) -> Self {
        TINYINT { value }
    }

    /// Converts the given i8 value to a 1-byte little-endian representation.
    pub fn to_bytes(&self) -> [u8; 1] {
        self.value.to_le_bytes()
    }

    /// Converts a 1-byte little-endian representation back to an i8 value.
    pub fn from_bytes(bytes: &[u8; 1]) -> Self {
        TINYINT {
            value: i8::from_le_bytes(*bytes),
        }
    }
}
