/// Represents a FLOAT (32-bit floating point) data type.
pub struct FLOAT {
    value: f32,
}

impl FLOAT {
    /// Creates a new FLOAT instance.
    pub fn new(value: f32) -> Self {
        FLOAT { value }
    }

    /// Converts the FLOAT value to a 4-byte little-endian representation.
    pub fn to_bytes(&self) -> [u8; 4] {
        self.value.to_le_bytes()
    }

    /// Converts a 4-byte little-endian representation back to a FLOAT value.
    pub fn from_bytes(bytes: &[u8; 4]) -> Self {
        FLOAT {
            value: f32::from_le_bytes(*bytes),
        }
    }
}
