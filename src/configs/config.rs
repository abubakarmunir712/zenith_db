/// Configuration module containing global constants for the database system.
///
/// This module defines various constants used throughout the project.
///
/// # Constants
/// - `PAGE_SIZE`: The size of a single page in bytes (default: 4096).
///
/// These constants help maintain consistency and avoid magic numbers in the code.
///

pub mod Config {
    pub const DB_PATH: &str = "";
    pub const PAGE_SIZE: u32 = 4096; // Size of page in bytes
    pub const PAGE_HEADER_SIZE: u16 = 20; // Size of page header in bytes
    pub const INDEX_PAGE_SIZE: u16 = 4096; // Size of page header in bytes
    pub const FSM_PAGE_SIZE: u16 = 4096; // Size of page header in bytes

}
