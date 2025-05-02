use crate::configs::types_config::TypesConfig::{MAX_CHAR_SIZE, MIN_CHAR_SIZE};
use crate::enums::type_errors::CharError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VARCHAR {
    pub size: u32,     // Maximum size of the VARCHAR field
    pub value: String, // The actual value of the VARCHAR field
}

impl VARCHAR {
    /// Constructor to create a new VARCHAR with validation
    ///
    /// Creates a `VARCHAR` field with a maximum size and a string value.
    /// The string length must not exceed `size`, `MAX_CHAR_SIZE`, or be less than `MIN_CHAR_SIZE`.
    pub fn new(size: u32, value: &str) -> Result<Self, &str> {
        let length: u32 = value.len() as u32;
        if length > size {
            return Err(CharError::LengthOverflow.message());
        }
        if length > MAX_CHAR_SIZE || length < MIN_CHAR_SIZE {
            return Err(CharError::SysLengthLimitExceeded.message());
        }
        Ok(VARCHAR {
            size,
            value: value.to_string(),
        })
    }

    /// Convert the VARCHAR struct to a Vec<u8> (binary representation)
    ///
    /// Serializes the string value to its UTF-8 bytes without a length prefix or padding.
    /// The length is assumed to be stored externally.
    pub fn to_bytes(&self) -> Vec<u8> {
        // Return only the string's UTF-8 bytes
        self.value.as_bytes().to_vec()
    }

    /// Convert a Vec<u8> back into a VARCHAR struct
    ///
    /// Deserializes UTF-8 bytes into a `VARCHAR` with the given maximum `size`.
    /// Expects raw string bytes without a length prefix. Validates that the byte length
    /// does not exceed `size` or `MAX_CHAR_SIZE`, and is at least `MIN_CHAR_SIZE`.
    pub fn from_bytes(bytes: &[u8], size: u32) -> Result<Self, &str> {
        let bytes_length: u32 = bytes.len() as u32;

        // Validate byte length against size constraints
        if bytes_length > MAX_CHAR_SIZE || bytes_length < MIN_CHAR_SIZE || bytes_length > size {
            return Err(CharError::SysLengthLimitExceeded.message());
        }

        // Convert bytes to a UTF-8 string
        let value = String::from_utf8(bytes.to_vec()).map_err(|_| CharError::InvalidUtf8.message())?;

        Ok(VARCHAR { size, value })
    }

    // Getter for the value field (returns the string value)
    pub fn value(&self) -> &str {
        &self.value
    }

    // Getter for the size field (returns the maximum size of the VARCHAR field)
    pub fn size(&self) -> u32 {
        self.size
    }
}
