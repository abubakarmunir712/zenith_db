/// Errors related to numeric and float data types.
#[derive(Debug)]
pub enum NumericError {
    /// Occurs when the format of the data is invalid or doesn't match the expected type.
    /// Example: Trying to insert a non-numeric string into an integer column.
    InvalidFormat,

    /// Occurs when numerical values exceed or fall short of the valid range for the specified data type.
    /// Example: Inserting a value that is too large or too small for an `TINYINT` column.
    OutOfRange,
}

impl NumericError {
    /// Returns a raw error message that describes the specific numeric error.
    pub fn message(&self) -> &str {
        match self {
            NumericError::InvalidFormat => "Invalid Format",
            NumericError::OutOfRange => "Value out of range",
        }
    }
}

/// Errors related to decimal data types.
#[derive(Debug)]
pub enum DecimalError {
    /// Occurs when the input exceeds the allowed precision for decimal numbers.
    /// Example: A decimal number with more digits than the allowed precision for the data type.
    PrecisionOverflow,

    /// Occurs when the input exceeds the system-defined precision limit.
    /// Example: A decimal value with precision greater than the system's capacity.
    PrecisionLimitExceeded,

    /// Occurs when the scale (number of decimal places) is invalid or out of bounds.
    /// Example: Trying to insert a decimal with more than the allowed number of decimal places.
    InvalidScale,

    /// Occurs when there is an attempt to divide by zero in a decimal operation.
    /// Example: Performing a division operation where the divisor is zero.
    DivisionByZero,

    /// Occurs when an arithmetic operation results in an overflow in a decimal calculation.
    /// Example: An addition or multiplication operation results in a value too large to be stored.
    ArithmeticOverflow,

    /// Occurs when an operation results in a loss of precision beyond the allowed limits.
    /// Example: Truncating a decimal value beyond the allowed precision.
    LossOfPrecision,

    /// Occurs when attempting to perform operations on decimals with mismatched scales.
    /// Example: Adding two decimal numbers with different scales (e.g., `1.25` and `1.3`).
    MismatchedScale,
}

#[rustfmt::skip]
impl DecimalError {
    /// Returns a human-readable message describing the specific decimal error.
    pub fn message(&self) -> &str {
        match self {
            DecimalError::PrecisionOverflow => "Input exceeds the allowed precision",
            DecimalError::PrecisionLimitExceeded => "Input exceeds the system-defined precision limit",
            DecimalError::InvalidScale => "Scale (decimal places) is invalid or out of bounds",
            DecimalError::DivisionByZero => "Attempted to divide by zero",
            DecimalError::ArithmeticOverflow => "Arithmetic operation resulted in an overflow",
            DecimalError::LossOfPrecision => "Operation caused loss of precision beyond allowed limits",
            DecimalError::MismatchedScale => "Operation attempted with decimals of mismatched scales",
        }
    }
}

/// Errors related to string (character) data types.
#[derive(Debug)]
pub enum StringError {
    /// Occurs when a string contains invalid UTF-8 sequences, which cannot be interpreted correctly.
    /// Example: Inserting a string that contains corrupted or invalid UTF-8 data.
    InvalidUtf8,

    /// Occurs when attempting to store a string that exceeds the system's maximum allowed length.
    /// Example: Trying to insert a string longer than the database or system can handle.
    SysLengthExceeded,

    /// Occurs when the string exceeds a user-defined length limit.
    /// Example: Trying to insert a string longer than the allowed column limit.
    LengthExceeded,
}

#[rustfmt::skip]
impl StringError {
    /// Returns a human-readable message describing the specific string error.
    pub fn message(&self) -> &str {
        match self {
            StringError::InvalidUtf8 => "String contains invalid UTF-8 sequences",
            StringError::SysLengthExceeded => "String exceeds the system-defined maximum length",
            StringError::LengthExceeded => "String exceeds the allowed column length",
        }
    }
}

/// Errors related to datetime data types.
#[derive(Debug)]
pub enum DateTimeError {
    /// Occurs when the date format is invalid.
    /// Example: A string not matching `YYYY-DD-MM`.
    InvalidDateFormat,

    /// Occurs when the date value is logically incorrect.
    /// Example: `2023-02-30`.
    InvalidDateValue,

    /// Occurs when the time format is invalid.
    /// Example: A string not matching `HH:MM:SS`.
    InvalidTimeFormat,

    /// Occurs when the time value is logically incorrect.
    /// Example: `25:00:00`.
    InvalidTimeValue,

    /// Occurs when the combined datetime string format is invalid.
    /// Expected: `YYYY-DD-MM HH:MM:SS`.
    InvalidDateTime,
}

impl DateTimeError {
    /// Returns a human-readable message describing the specific datetime error.
    pub fn message(&self) -> &str {
        match self {
            DateTimeError::InvalidDateFormat => "Invalid date format (expected YYYY-MM-DD).",
            DateTimeError::InvalidDateValue => "Date is out of valid range or logically incorrect.",
            DateTimeError::InvalidTimeFormat => "Invalid time format (expected HH:MM:SS).",
            DateTimeError::InvalidTimeValue => "Time is out of valid range or logically incorrect.",
            DateTimeError::InvalidDateTime => {
                "Invalid datetime format (expected YYYY-MM-DD HH:MM:SS)."
            }
        }
    }
}

/// Errors related to bool data types.
#[derive(Debug)]
pub enum BoolError {
    /// Occurs when a value other than `true` or `false` is provided for a boolean.
    InvalidValue,
}

impl BoolError {
    /// Returns a human-readable message describing the specific boolean error.
    pub fn message(&self) -> &str {
        match self {
            BoolError::InvalidValue => "Only true or false are allowed.",
        }
    }
}

pub enum ValidationError {
    CannotBeNull,
}

impl ValidationError {
    pub fn message(&self) -> &str {
        match self {
            ValidationError::CannotBeNull => "Null not allowed",
        }
    }
}
