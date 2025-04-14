/// Represents an INT (32-bit signed integer) data type.
pub struct INT {
    value: i32,
}

impl INT {
    /// Creates a new INT instance.
    pub fn new(value: i32) -> Self {
        INT { value }
    }

    /// Converts the given i32 value to a 4-byte little-endian representation.
    pub fn to_bytes(&self) -> [u8; 4] {
        self.value.to_le_bytes()
    }

    /// Converts a 4-byte little-endian representation back to an i32 value.
    pub fn from_bytes(bytes: &[u8; 4]) -> Self {
        INT {
            value: i32::from_le_bytes(*bytes),
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
