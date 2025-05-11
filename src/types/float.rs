/// Represents a FLOAT (32-bit floating point) data type.
use serde::{Deserialize, Serialize};

use crate::enums::errors::type_errors::NumericError;

#[derive(Serialize, Deserialize, Debug)]
pub struct FLOAT {
    value: f32,
}

impl FLOAT {
    /// Creates a new FLOAT instance.
    pub fn new(value: &str) -> Result<Self, &str> {
        let value = value.parse::<f32>();
        match value {
            Ok(val) => {
                if val.is_infinite() || val.is_nan() {
                    Err(NumericError::OutOfRange.message())
                } else {
                    Ok(FLOAT { value: val })
                }
            }
            Err(_) => Err(NumericError::InvalidFormat.message()),
        }
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

    pub fn value(&self) -> f32 {
        self.value
    }
}
