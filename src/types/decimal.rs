use crate::enums::errors::type_errors::{DecimalError, NumericError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DECIMAL {
    value: i128,
    scale: u32,
    precision: u32,
}

impl DECIMAL {
    pub fn new(value: &str, precision: u32, scale: u32) -> Result<Self, &str> {
        let is_signed: u32 = if value.starts_with('-') || value.starts_with('+') {
            1
        } else {
            0
        };

        let mut parts: Vec<&str> = value.split(".").collect();

        if parts.len() > 2 {
            return Err(NumericError::InvalidFormat.message());
        }

        if !Self::_is_numeric(parts[0]) {
            return Err(NumericError::InvalidFormat.message());
        }

        if parts[0].len() as u32 > (precision - scale + is_signed) {
            return Err(DecimalError::PrecisionOverflow.message());
        }

        let mut dec_part_len: u32 = if parts.len() > 1 {
            if !Self::_is_numeric(parts[1]) {
                return Err(NumericError::InvalidFormat.message());
            }
            parts[1].len() as u32
        } else {
            0
        };

        if dec_part_len > scale {
            parts[1] = &parts[1][..(scale+1) as usize];
            dec_part_len = parts[1].len() as u32;
        }

        let mut value = parts.join("");

        while scale > dec_part_len {
            value += "0";
            dec_part_len += 1;
        }

        let mut value: i128 = value
            .parse()
            .map_err(|_| NumericError::InvalidFormat.message())?;
        let is_overflowed = Self::handle_rounding(&mut value, scale, dec_part_len);
        if is_overflowed {
            return Err(DecimalError::PrecisionOverflow.message());
        }
        Ok(DECIMAL {
            value,
            scale,
            precision,
        })
    }

    fn handle_rounding(value: &mut i128, scale: u32, dec_part_len: u32) -> bool {
        let mut is_overflowed = false;
        if dec_part_len <= scale {
            return is_overflowed;
        }
        *value = *value / 10i128.pow(dec_part_len - scale - 1);
        let last_digit = *value % 10;
        *value = *value / 10;
        if last_digit.abs() >= 5 {
            let digits_before_rounding = Self::_count_digits(*value);
            if *value >= 0{

                *value += 1;
            }
            else{
                *value-=1;
            }
            let digits_after_rounding = Self::_count_digits(*value);
            is_overflowed = digits_after_rounding > digits_before_rounding;
        }
        return is_overflowed;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }

    /// Converts an 8-byte little-endian representation back to a DOUBLE value.
    pub fn from_bytes(bytes: &[u8], precision: u32, scale: u32) -> Self {
        let bytes: [u8; 16] = bytes.try_into().unwrap();
        DECIMAL {
            value: i128::from_le_bytes(bytes),
            precision,
            scale,
        }
    }

    fn _is_numeric(s: &str) -> bool {
        let mut iter = s.chars();

        // Check if the first character is either a digit, '+' or '-'
        if let Some(first_char) = iter.next() {
            if !first_char.is_digit(10) && first_char != '+' && first_char != '-' {
                return false;
            }
        }
        // Ensure the rest of the string only contains digits
        for c in iter {
            if !c.is_digit(10) {
                return false;
            }
        }
        true
    }

    fn _count_digits(n: i128) -> u32 {
        if n == 0 {
            return 1;
        }
        let mut n = n.abs() as u128;
        let mut count = 0;
        while n > 0 {
            count += 1;
            n /= 10;
        }
        count
    }

    pub fn value(&self) -> i128 {
        self.value
    }

    pub fn value_string(&self)->String{
        let mut val = self.value.to_string();
        val.insert(val.len()-self.scale as usize, '.');
        val
    }
    pub fn scale(&self) -> u32 {
        self.scale
    }
    pub fn precision(&self) -> u32 {
        self.precision
    }
}
