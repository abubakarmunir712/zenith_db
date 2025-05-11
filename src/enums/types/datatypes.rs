use crate::{configs::types_config::TypesConfig::MAX_TEXT_SIZE, types::null::NULL};

pub enum DataType {
    CHAR,     // Fixed-length character type
    VARCHAR,  // Variable-length character type
    BOOL,     // Boolean type (true/false)
    INT,      // Integer type
    BIGINT,   // Big integer type
    SMALLINT, // Small integer type
    TINYINT,  // Tiny integer type
    DECIMAL,  // Decimal type (fixed-point number)
    DOUBLE,   // Double precision floating-point number
    FLOAT,    // Single precision floating-point number
    DATE,     // Date type
    TIME,     // Time type
    DATETIME, // Combined Date and Time type
    TEXT,     // Text type (longer string)
    NULL,
}

impl DataType {
    /// Maps each data type to a unique OID (used for internal representation).
    pub fn to_oid(&self) -> u8 {
        match self {
            DataType::CHAR => 0,
            DataType::VARCHAR => 1,
            DataType::BOOL => 2,
            DataType::INT => 3,
            DataType::BIGINT => 4,
            DataType::SMALLINT => 5,
            DataType::TINYINT => 6,
            DataType::DECIMAL => 7,
            DataType::DOUBLE => 8,
            DataType::FLOAT => 9,
            DataType::DATE => 10,
            DataType::TIME => 11,
            DataType::DATETIME => 12,
            DataType::TEXT => 13,
            DataType::NULL => 14,
        }
    }

    /// Converts an OID back to the corresponding DataType.
    /// This is safe because OIDs are internally generated and trusted.
    pub fn from_oid(oid: u8) -> Self {
        match oid {
            0 => DataType::CHAR,
            1 => DataType::VARCHAR,
            2 => DataType::BOOL,
            3 => DataType::INT,
            4 => DataType::BIGINT,
            5 => DataType::SMALLINT,
            6 => DataType::TINYINT,
            7 => DataType::DECIMAL,
            8 => DataType::DOUBLE,
            9 => DataType::FLOAT,
            10 => DataType::DATE,
            11 => DataType::TIME,
            12 => DataType::DATETIME,
            13 => DataType::TEXT,
            14 => DataType::NULL,
            _=>unreachable!()
        }
    }

    /// Returns the size (in bytes) for fixed-size types.
    ///
    /// For CHAR, VARCHAR, and DECIMAL, the size is **user-defined** and should be passed externally.
    pub fn size(&self) -> u32 {
        match self {
            DataType::BOOL => 1,
            DataType::INT => 4,
            DataType::BIGINT => 8,
            DataType::SMALLINT => 2,
            DataType::TINYINT => 1,
            DataType::DOUBLE => 8,
            DataType::FLOAT => 4,
            DataType::DATE => 4,
            DataType::TIME => 3,
            DataType::DATETIME => 7,
            DataType::TEXT => MAX_TEXT_SIZE,
            DataType::NULL => 0,
            _ => unreachable!(),
        }
    }
}
