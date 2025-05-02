#[derive(Debug)]
pub enum TypeError {
    /// Occurs when there is a mismatch between expected and actual data types.
    MismatchedDataType,
}
impl TypeError {
    pub fn message(&self, value: &str, datatype: &str) -> String {
        match self {
            TypeError::MismatchedDataType => {
                format!(
                    "Mismatched data type: expected '{}', found '{}'",
                    datatype, value
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum DecimalError {
    /// Occurs when the input exceeds the allowed precision..
    PrecisionOverflow,

    /// Occurs when the input exceeds the system-defined precision limit.
    SysPrecisionLimitExceeded,

    /// Occurs when scale (decimal places) is invalid or out of bounds.
    InvalidScale,

    /// Occurs when attempting to divide by zero.
    DivisionByZero,

    /// Occurs when parsing a string into a decimal fails due to invalid format.
    InvalidFormat,

    /// Occurs when an arithmetic operation results in an overflow.
    ArithmeticOverflow,

    /// Occurs when an operation results in loss of precision beyond allowed limits.
    LossOfPrecision,

    /// Occurs when trying to perform operations on decimals with mismatched scales.
    MismatchedScale,
}

#[rustfmt::skip]
impl DecimalError {
    /// Returns a human-readable message describing the error.
    pub fn message(&self) -> &str {
        match self {
            DecimalError::PrecisionOverflow => "Input exceeds the allowed precision",
            DecimalError::SysPrecisionLimitExceeded => "Input exceeds the system-defined precision limit",
            DecimalError::InvalidScale => "Scale (decimal places) is invalid or out of bounds",
            DecimalError::DivisionByZero => "Attempted to divide by zero",
            DecimalError::InvalidFormat => "Failed to parse string due to invalid decimal format",
            DecimalError::ArithmeticOverflow => "Arithmetic operation resulted in an overflow",
            DecimalError::LossOfPrecision => "Operation caused loss of precision beyond allowed limits",
            DecimalError::MismatchedScale => "Operation attempted with decimals of mismatched scales"
        }
    }
}

#[derive(Debug)]
pub enum CharError {
    /// Occurs when attempting to store a string exceeding the allowed length.
    LengthOverflow,

    /// Occurs when a string contains invalid UTF-8 sequences.
    InvalidUtf8,

    /// Occurs when attempting to store a string exceeding the **system-defined** maximum length.
    SysLengthLimitExceeded,

    /// Occurs when the binary array or vector is below or above bounds during deserialization.
    InvalidBinary,
}

#[rustfmt::skip]
impl CharError {
    /// Returns a human-readable message describing the error.
    pub fn message(&self) -> &str {
        match self {
            CharError::LengthOverflow => "String exceeds the allowed length",
            CharError::InvalidUtf8 => "String contains invalid UTF-8 sequences",
            CharError::SysLengthLimitExceeded => "String exceeds the system-defined maximum length",
            CharError::InvalidBinary => "Binary data is invalid or out of bounds during deserialization"
        }
    }
}

#[derive(Debug)]
pub enum DateTimeError {
    /// Occurs when the date/time format is incorrect.
    InvalidFormat,

    /// Triggered when the date/time value is invalid.
    InvalidValue,
}

#[rustfmt::skip]
impl DateTimeError {
    /// Returns a human-readable message describing the error.
    pub fn message(&self) -> &str {
        match self {
            DateTimeError::InvalidFormat => "The provided date/time format is invalid.",
            DateTimeError::InvalidValue => "The date/time value is out of valid range or logically incorrect."
        }
    }
}
