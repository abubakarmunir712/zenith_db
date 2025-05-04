/// Represents a TINYINT (8-bit signed integer) data type.
use serde::{Deserialize, Serialize};

use crate::enums::type_errors::NumericError;

#[derive(Serialize, Deserialize, Debug)]
pub struct TINYINT {
    value: i8,
}

impl TINYINT {
    /// Creates a new TINYINT instance.
    pub fn new(value: &str) -> Result<Self, &str> {
        let value = value.parse::<i8>();
        match value {
            Ok(val) => Ok(TINYINT { value: val }),
            Err(e) => {
                if e.to_string().contains("too large") {
                    Err(NumericError::OutOfRange.message())
                } else {
                    Err(NumericError::InvalidFormat.message())
                }
            }
        }
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

    pub fn value(&self) -> i8 {
        self.value
    }
}
