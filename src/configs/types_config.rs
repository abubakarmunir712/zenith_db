/// Configuration module for type-related constants.
///
/// This module defines size limits for various data types used in the database system.
/// Keeping these values centralized ensures consistency across the project and
/// prevents hardcoded "magic numbers" in the codebase.
///
/// # Constants
/// - `MAX_DECIMAL_PRECISION`: The maximum precision allowed for DECIMAL types (default: 38).
/// - `MIN_DECIMAL_PRECISION`: The minimum precision allowed for DECIMAL types (default: 1).
/// - `MAX_CHAR_SIZE`: The maximum allowed size for a CHAR field (default: 65536).
/// - `MIN_CHAR_SIZE`: The minimum allowed size for a CHAR field (default: 1).
///
/// These constraints help enforce storage limitations and maintain database integrity.

pub mod TypesConfig {
    pub const MAX_DECIMAL_PRECISION: u32 = 38;
    pub const MIN_DECIMAL_PRECISION: u32 = 1;
    pub const MAX_CHAR_SIZE: u32 = 65536;
    pub const MIN_CHAR_SIZE: u32 = 1;
}
