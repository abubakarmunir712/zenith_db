/// Represents a SMALLINT (16-bit signed integer) data type.
use serde::{Deserialize, Serialize};

use crate::enums::errors::type_errors::NumericError;

#[derive(Serialize, Deserialize, Debug)]
pub struct SMALLINT {
    value: i16,
}

impl SMALLINT {
    /// Creates a new SMALLINT instance.
    pub fn new(value: &str) -> Result<Self, &str> {
        let value = value.parse::<i16>();
        match value {
            Ok(val) => Ok(SMALLINT { value: val }),
            Err(e) => {
                if e.to_string().contains("too large") {
                    Err(NumericError::OutOfRange.message())
                } else {
                    Err(NumericError::InvalidFormat.message())
                }
            }
        }
    }
    /// Converts the given i16 value to a 2-byte little-endian representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    /// Converts a 2-byte little-endian representation back to an i16 value.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 2] = bytes.try_into().unwrap();
        SMALLINT {
            value: i16::from_le_bytes(bytes),
        }
    }

    pub fn value(&self) -> i16 {
        self.value
    }
}
