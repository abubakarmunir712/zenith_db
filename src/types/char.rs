use crate::configs::types_config::TypesConfig::{MAX_CHAR_SIZE, MIN_CHAR_SIZE};
use crate::enums::type_errors::CharError;

pub struct CHAR {
    pub size: u32, // Size of the CHAR field (can be used for both CHAR and VARCHAR depending on the padding)
    pub value: String, // The actual value of the CHAR field
}

impl CHAR {
    /// Constructor to create a new CHAR with validation
    ///
    /// This can be used for both `CHAR` and `VARCHAR` types by adjusting the padding.
    /// For `CHAR`, padding will be added to ensure the field is exactly `size` bytes long.
    /// For `VARCHAR`, no padding is applied, and the string length will be respected.
    pub fn new(size: u32, value: &str) -> Result<Self, CharError> {
        let length: u32 = value.len() as u32;
        if length > MAX_CHAR_SIZE || length < MIN_CHAR_SIZE || length > size {
            return Err(CharError::SysLengthLimitExceeded);
        }
        Ok(CHAR {
            size,
            value: value.to_string(),
        })
    }

    /// Convert the CHAR struct to a Vec<u8> (binary representation)
    ///
    /// - If `allow_padding` is true, this function behaves like a `CHAR` and pads the
    ///   string to meet the specified size.
    /// - If `allow_padding` is false, it behaves like a `VARCHAR` and the string will
    ///   not be padded.
    pub fn to_bytes(&self, allow_padding: bool) -> Vec<u8> {
        let mut result = Vec::new();

        // Add the length of the string (4 bytes for the length)
        result.extend_from_slice(&(self.value.len() as u32).to_le_bytes());

        // Add the actual string as bytes (UTF-8 encoded)
        result.extend_from_slice(self.value.as_bytes());

        // If padding is allowed (for CHAR), ensure the string length occupies the specified 'size'
        let padding_len = self.size as usize - self.value.len();

        // Add padding if the value is shorter than the size and if padding is allowed
        if padding_len > 0 && allow_padding {
            result.extend(vec![0u8; padding_len]);
        }

        result
    }

    /// Convert a Vec<u8> back into a CHAR struct
    ///
    /// This function can be used for both `CHAR` and `VARCHAR` types depending on the padding.
    /// For `CHAR`, it assumes the length is exactly `size` bytes, while for `VARCHAR`, it
    /// will expect a variable length string without padding.
    pub fn from_bytes(bytes: Vec<u8>, size: u32) -> Result<Self, CharError> {
        let bytes_length: usize = bytes.len();
        // There must be at least 5 bytes (4 for size and 1 for value)
        if bytes_length < 5 || bytes_length > MAX_CHAR_SIZE as usize + 2 {
            return Err(CharError::InvalidBinary);
        }

        // Extract the length (first 4 bytes)
        let length = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        if length > size as usize {
            return Err(CharError::LengthOverflow);
        }

        // Ensure the length matches the rest of the bytes in the Vec
        if bytes.len() - 4 > length {
            return Err(CharError::LengthOverflow);
        }

        // Extract the actual value from the bytes
        let value =
            String::from_utf8(bytes[4..4 + length].to_vec()).map_err(|_| CharError::InvalidUtf8)?;

        Ok(CHAR { size, value })
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
