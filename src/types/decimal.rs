use crate::configs::types_config::TypesConfig::{MAX_DECIMAL_PRECISION, MIN_DECIMAL_PRECISION};
use crate::enums::type_errors::DecimalError;
pub struct DECIMAL {
    value: i128,
    scale: u32,
    precision: u32,
}

impl DECIMAL {
    pub fn new(value: &str, precision: u32, scale: u32) -> Result<Self, DecimalError> {
        let is_signed: u32 = if value.starts_with('-') || value.starts_with('+') {
            1
        } else {
            0
        };

        if scale > precision {
            return Err(DecimalError::InvalidScale);
        }
        // Max allowed precision is 38, because i128 can only hold upto 38 digits!
        if precision < MIN_DECIMAL_PRECISION || precision > MAX_DECIMAL_PRECISION {
            return Err(DecimalError::SysPrecisionLimitExceeded);
        }

        let parts: Vec<&str> = value.split(".").collect();

        if parts[0].len() as u32 > (precision - scale + is_signed) {
            return Err(DecimalError::PrecisionOverflow);
        }

        if parts.len() > 2 {
            return Err(DecimalError::InvalidFormat);
        }

        let mut dec_part_len: u32 = if parts.len() > 1 {
            parts[1].len() as u32
        } else {
            0
        };

        if parts[0].len() as u32 + dec_part_len - is_signed > MAX_DECIMAL_PRECISION {
            return Err(DecimalError::PrecisionOverflow);
        }

        let mut value = parts.join("");

        while scale > dec_part_len {
            value += "0";
            dec_part_len += 1;
        }

        let mut value: i128 = value.parse().map_err(|_| DecimalError::InvalidFormat)?;
        Self::handle_rounding(&mut value, scale, dec_part_len);
        Ok(DECIMAL {
            value,
            scale,
            precision,
        })
    }

    fn handle_rounding(value: &mut i128, scale: u32, dec_part_len: u32) {
        if dec_part_len <= scale {
            return;
        }
        *value = *value / 10i128.pow(dec_part_len - scale - 1);
        let last_digit = *value % 10;
        *value = *value / 10;
        if last_digit >= 5 {
            *value += 1;
        }
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        self.value.to_le_bytes()
    }

    /// Converts an 8-byte little-endian representation back to a DOUBLE value.
    pub fn from_bytes(bytes: &[u8; 16], precision: u32, scale: u32) -> Self {
        DECIMAL {
            value: i128::from_le_bytes(*bytes),
            precision,
            scale,
        }
    }

    pub fn value(&self) -> i128 {
        self.value
    }
    pub fn scale(&self) -> u32 {
        self.scale
    }
    pub fn precision(&self) -> u32 {
        self.precision
    }
}
