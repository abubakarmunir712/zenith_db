#[derive(Debug)]
pub enum DecimalError {
    /// Occurs when the input exceeds the allowed precision..
    PrecisionOverflow,

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
