/// Represents a BIGINT (64-bit signed integer) data type.
use serde::{Deserialize, Serialize};

use crate::enums::type_errors::NumericError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BIGINT {
    pub value: i64,
}

impl BIGINT {
    /// Creates a new BIGINT instance.
    pub fn new(value: &str) -> Result<Self, &str> {
        let value = value.parse::<i64>();
        match value {
            Ok(val) => Ok(BIGINT { value: val }),
            Err(e) => {
                if e.to_string().contains("too large") {
                    Err(NumericError::OutOfRange.message())
                } else {
                    Err(NumericError::InvalidFormat.message())
                }
            }
        }
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
