// Define a struct `BOOL` to represent a boolean value.
use serde::{Deserialize, Serialize};

use crate::enums::errors::type_errors::BoolError;

#[derive(Serialize, Deserialize, Debug)]
pub struct BOOL {
    value: bool, // Holds the actual boolean value.
}

impl BOOL {
    // Constructor: Creates a new BOOL instance with the given value.
    pub fn new(value: &str) -> Result<Self, &str> {
        if value.to_lowercase() == "false" {
            Ok(Self { value: false })
        } else if value.to_lowercase() == "true" {
            Ok(Self { value: true })
        } else {
            Err(BoolError::InvalidValue.message())
        }
    }

    // Converts the BOOL value to a single byte (1 for true, 0 for false).
    pub fn to_bytes(&self) -> Vec<u8> {
        if self.value {
            return vec![1]; // Return an vec containing 1 if true.
        }
        return vec![0]; // Return an vec containing 0 if false.
    }

    // Converts a byte array back into a BOOL instance.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        if bytes[0] == 0 {
            return Self { value: false }; // If byte is 0, set value to false.
        }
        return Self { value: true }; // Otherwise, set value to true.
    }
    pub fn value(&self) -> bool {
        self.value
    }
}
