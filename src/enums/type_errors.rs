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

#[derive(Debug)]
pub enum DateTimeError {
    /// Occurs when the date/time format is incorrect.
    InvalidFormat,

    /// Triggered when the date/time value is invalid.
    InvalidValue,
}
