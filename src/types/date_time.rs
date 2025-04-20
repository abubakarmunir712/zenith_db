use crate::enums::type_errors::DateTimeError;

use super::{date::DATE, time::TIME};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]

pub struct DATETIME {
    date: DATE,
    time: TIME,
}

impl DATETIME {
    pub fn new(date_time: &str) -> Result<Self, DateTimeError> {
        let parts: Vec<&str> = date_time.split(" ").collect();
        if parts.len() != 2 {
            return Err(DateTimeError::InvalidFormat);
        }
        let date = DATE::new(parts[0])?;
        let time = TIME::new(parts[1])?;

        Ok(Self { date, time })
    }

    pub fn to_bytes(&self) -> [u8; 7] {
        let mut result: [u8; 7] = [0; 7];
        result[0..4].copy_from_slice(&self.date.to_bytes());
        result[4..7].copy_from_slice(&self.time.to_bytes());
        result
    }

    pub fn from_bytes(bytes: &[u8; 7]) -> Self {
        let date = DATE::from_bytes(&bytes[0..4].try_into().unwrap());
        let time = TIME::from_bytes(&bytes[4..7].try_into().unwrap());

        DATETIME { date, time }
    }

    pub fn date(&self) -> &DATE {
        &self.date
    }

    pub fn time(&self) -> &TIME {
        &self.time
    }

    pub fn value(&self) -> String {
        self.date.value() + " " + &self.time.value()
    }
}
