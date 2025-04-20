/// Represents a DOUBLE (64-bit floating point) data type.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct DOUBLE {
    value: f64,
}

impl DOUBLE {
    /// Creates a new DOUBLE instance.
    pub fn new(value: f64) -> Self {
        DOUBLE { value }
    }

    /// Converts the DOUBLE value to an 8-byte little-endian representation.
    pub fn to_bytes(&self) -> [u8; 8] {
        self.value.to_le_bytes()
    }

    /// Converts an 8-byte little-endian representation back to a DOUBLE value.
    pub fn from_bytes(bytes: &[u8; 8]) -> Self {
        DOUBLE {
            value: f64::from_le_bytes(*bytes),
        }
    }
}
