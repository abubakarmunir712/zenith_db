use crate::{
    configs::types_config::TypesConfig::{MAX_CHAR_SIZE, MIN_CHAR_SIZE},
    enums::type_errors::StringError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CHAR {
    size: u32, // Size of the CHAR field (can be used for both CHAR and VARCHAR depending on the padding)
    value: String, // The actual value of the CHAR field
}

impl CHAR {
    /// Constructor to create a new CHAR with validation
    pub fn new(size: u32, value: &str) -> Result<Self, &str> {
        let length: u32 = value.len() as u32;
        if length > size {
            return Err(StringError::LengthExceeded.message());
        }
        if length > MAX_CHAR_SIZE || length < MIN_CHAR_SIZE {
            return Err(StringError::SysLengthExceeded.message());
        }
        Ok(CHAR {
            size,
            value: value.to_string(),
        })
    }

    /// Convert the CHAR struct to a Vec<u8> (binary representation)
    ///
    /// Serializes data by prefixing it with a 4-byte length,
    /// Followed by the data, and pads the result to a total length of `max_size + 4` bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Add the length of the string (4 bytes for the length)
        result.extend_from_slice(&(self.value.len() as u32).to_le_bytes());

        // Add the actual string as bytes (UTF-8 encoded)
        result.extend_from_slice(self.value.as_bytes());

        // Ensure the string length occupies the specified 'size'
        let padding_len = self.size as usize - self.value.len();

        // Add padding if the value is shorter than the size and if padding is allowed
        if padding_len > 0 {
            result.extend(vec![0u8; padding_len]);
        }

        result
    }

    /// Convert a Vec<u8> back into a CHAR struct
    pub fn from_bytes(bytes: &[u8], size: u32) -> Self {
        // Extract the length (first 4 bytes)
        let length = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        // Extract the actual value from the bytes
        let value = String::from_utf8(bytes[4..4 + length].to_vec()).unwrap();
        CHAR { size, value }
    }

    // Getter for the value field (returns the string value)
    pub fn value(&self) -> &str {
        &self.value
    }

    // Getter for the size field (returns the size of the CHAR field)
    pub fn size(&self) -> u32 {
        self.size
    }
}
