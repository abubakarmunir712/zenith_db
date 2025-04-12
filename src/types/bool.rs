// Define a struct `BOOL` to represent a boolean value.
pub struct BOOL {
    pub value: bool, // Holds the actual boolean value.
}

impl BOOL {
    // Constructor: Creates a new BOOL instance with the given value.
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    // Converts the BOOL value to a single byte (1 for true, 0 for false).
    pub fn to_bytes(&self) -> [u8; 1] {
        if self.value {
            return [1]; // Return an array containing 1 if true.
        }
        return [0]; // Return an array containing 0 if false.
    }

    // Converts a byte array back into a BOOL instance.
    pub fn from_bytes(bytes: &[u8; 1]) -> Self {
        if bytes[0] == 0 {
            return Self { value: false }; // If byte is 0, set value to false.
        }
        return Self { value: true }; // Otherwise, set value to true.
    }
}
