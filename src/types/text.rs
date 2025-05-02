use crate::configs::types_config::TypesConfig::{MAX_TEXT_SIZE, MIN_TEXT_SIZE};
use crate::enums::type_errors::CharError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct TEXT {
    value: String, // The actual text value
}

impl TEXT {
    /// Constructor to create a new TEXT with validation
    pub fn new(value: &str) -> Result<Self, &str> {
        let length: u32 = value.len() as u32;
        if length > MAX_TEXT_SIZE || length < MIN_TEXT_SIZE {
            return Err(CharError::SysLengthLimitExceeded.message());
        }
        Ok(TEXT {
            value: value.to_string(),
        })
    }

    /// Convert the TEXT struct to a Vec<u8> (binary representation)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Store the length of the text (first 4 bytes)
        result.extend_from_slice(&(self.value.len() as u32).to_le_bytes());

        // Store the actual text as bytes (UTF-8 encoded)
        result.extend_from_slice(self.value.as_bytes());

        result
    }

    /// Convert a Vec<u8> back into a TEXT struct
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &str> {
        let bytes_length = bytes.len();
        // There must be at least 5 bytes (4 for size + 1 for content)
        if bytes_length < 5 || bytes_length > MAX_TEXT_SIZE as usize + 4 {
            return Err(CharError::InvalidBinary.message());
        }

        // Extract the length (first 4 bytes)
        let length = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        // Ensure the length matches the actual available bytes
        if bytes.len() - 4 != length {
            return Err(CharError::LengthOverflow.message());
        }

        // Extract the actual value from the bytes
        let value =
            String::from_utf8(bytes[4..4 + length].to_vec()).map_err(|_| CharError::InvalidUtf8.message())?;

        Ok(TEXT { value })
    }

    /// Getter for the value field
    pub fn value(&self) -> &str {
        &self.value
    }
}
