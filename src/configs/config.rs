/// Configuration module containing global constants for the database system.
///
/// This module defines various constants used throughout the project.
///
/// # Constants
/// - `DB_PATH`: Default path for the database files.
/// - `PAGE_SIZE`: Size (in bytes) of a standard data page.
/// - `PAGE_HEADER_SIZE`: Size (in bytes) of the header section of a page.
/// - `INDEX_PAGE_SIZE`: Size (in bytes) of an index page.
/// - `FSM_PAGE_SIZE`: Size (in bytes) of a Free Space Map (FSM) page.
/// - `CATLOG_PAGE_SIZE`: Size (in bytes) of a catalog page.
/// - `PAGE_BUF_CAP`: Initial buffer pool capacity in number of pages (e.g., 64MB / 4KB = 16384 pages).
///
pub mod Config {
    pub const DB_PATH: &str = "";
    pub const PAGE_SIZE: u32 = 4096;
    pub const PAGE_HEADER_SIZE: u16 = 20;
    pub const INDEX_PAGE_SIZE: u16 = 4096;
    pub const FSM_PAGE_SIZE: u16 = 4096;
    pub const CATLOG_PAGE_SIZE: u16 = 32768;
    pub const PAGE_BUF_CAP: u16 = 16384;
}
