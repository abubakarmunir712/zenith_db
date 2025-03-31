/// Represents a SMALLINT (16-bit signed integer) data type.
pub struct SMALLINT {
    value: i16,
}

impl SMALLINT {
    /// Creates a new SMALLINT instance.
    pub fn new(value: i16) -> Self {
        SMALLINT { value }
    }

    /// Converts the given i16 value to a 2-byte little-endian representation.
    pub fn to_bytes(&self) -> [u8; 2] {
        self.value.to_le_bytes()
    }

    /// Converts a 2-byte little-endian representation back to an i16 value.
    pub fn from_bytes(bytes: &[u8; 2]) -> i16 {
        i16::from_le_bytes(*bytes)
    }
}
