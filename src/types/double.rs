/// Represents a DOUBLE (64-bit floating point) data type.
use serde::{Deserialize, Serialize, de::value};

use crate::enums::errors::type_errors::NumericError;

#[derive(Serialize, Deserialize, Debug)]
pub struct DOUBLE {
    value: f64,
}

impl DOUBLE {
    /// Creates a new DOUBLE instance.
    pub fn new(value: &str) -> Result<Self, &str> {
        let value = value.parse::<f64>();
        match value {
            Ok(val) => {
                if val.is_infinite() || val.is_nan() {
                    Err(NumericError::OutOfRange.message())
                } else {
                    Ok(DOUBLE { value: val })
                }
            }
            Err(_) => Err(NumericError::InvalidFormat.message()),
        }
    }

    /// Converts the DOUBLE value to an 8-byte little-endian representation.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    /// Converts an 8-byte little-endian representation back to a DOUBLE value.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes: [u8; 8] = bytes.try_into().unwrap();
        DOUBLE {
            value: f64::from_le_bytes(bytes),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
