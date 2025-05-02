/// Represents a SMALLINT (16-bit signed integer) data type.
use serde::{Deserialize, Serialize};

use crate::enums::type_errors::TypeError;

#[derive(Serialize, Deserialize, Debug)]
pub struct SMALLINT {
    value: i16,
}

impl SMALLINT {
    /// Creates a new SMALLINT instance.
    pub fn new(value: &str) -> Result<Self, String> {
        let value: i16 = value
            .parse()
            .map_err(|e| TypeError::MismatchedDataType.message(value, "SMALLINT"))?;
        Ok(SMALLINT { value })
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

    pub fn value(&self)->i16{
        self.value
    }
}
