use crate::enums::type_errors::DateTimeError;

pub struct DATE {
    year: u16,
    day: u8,
    month: u8,
}

impl DATE {
    pub fn new(date: &str) -> Result<Self, DateTimeError> {
        let parts: Vec<&str> = date.split("-").collect();
        if parts.len() != 3 {
            return Err(DateTimeError::InvalidFormat);
        }
        let year = parts[0];
        let month = parts[1];
        let date = parts[2];

        if year.len() > 4 || month.len() > 2 || date.len() > 2 {
            return Err(DateTimeError::InvalidFormat);
        }

        let year: u16 = year.parse().map_err(|_| DateTimeError::InvalidFormat)?;
        let month: u8 = month.parse().map_err(|_| DateTimeError::InvalidFormat)?;
        let day: u8 = date.parse().map_err(|_| DateTimeError::InvalidFormat)?;

        if !(Self::is_date_valid(year, month, day)) {
            return Err(DateTimeError::InvalidValue);
        }

        Ok(Self { year, day, month })
    }

    pub fn is_year_leap(year: u16) -> bool {
        if year % 4 != 0 {
            return false;
        }
        if year % 100 == 0 && year % 400 != 0 {
            return false;
        }
        return true;
    }

    pub fn is_date_valid(year: u16, month: u8, day: u8) -> bool {
        let is_valid: bool = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                if day > 31 {
                    false
                } else {
                    true
                }
            }
            4 | 6 | 9 | 11 => {
                if day > 30 {
                    false
                } else {
                    true
                }
            }
            2 => {
                let is_year_leap = Self::is_year_leap(year);
                if (is_year_leap && day > 29) || (!is_year_leap && day > 28) {
                    false
                } else {
                    true
                }
            }
            _ => false,
        };
        is_valid
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let mut result: [u8; 4] = [0; 4];

        // Store the year (first 2 bytes)
        result.copy_from_slice(&self.year.to_le_bytes());
        result.copy_from_slice(&self.month.to_le_bytes());
        result.copy_from_slice(&self.day.to_le_bytes());

        result
    }

    pub fn from_bytes(bytes: &[u8; 4]) -> Self {
        let year = u16::from_le_bytes([bytes[0], bytes[1]]);
        let month = bytes[2];
        let day = bytes[3];

        DATE { year, month, day }
    }

    pub fn value(&self) -> String {
        (self.year.to_string()) + "-" + &(self.month.to_string()) + "-" + &(self.day.to_string())
    }

    pub fn month(&self) -> u8 {
        self.month
    }
    pub fn day(&self) -> u8 {
        self.day
    }
    pub fn year(&self) -> u16 {
        self.year
    }
}
