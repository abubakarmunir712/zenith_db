// page.rs - Defines the Page struct and its core functionalities.
//
// This file contains the definition of the Page struct, which represents
// a unit of storage in the database. It also includes important
// implementations for managing page lifecycle, serialization, and data
// manipulation.
//
// Pages are a fundamental part of the storage engine, interacting with
// the buffer pool and file I/O system.
//
pub struct Page {
    _is_dirty: bool,
    _data: Vec<u8>,
}
impl Page {
    /// Creates a new `Page` instance with the given values.
    ///
    /// This initializes a `Page` with:
    /// - A specified `is_dirty` flag.
    /// - A non-empty `Vec<u8>` for `data`.
    ///
    /// # Arguments
    /// - `is_dirty` – Indicates whether the page is dirty.
    /// - `data` – A vector of bytes that must not be empty.
    ///
    /// # Returns
    /// - `Ok(Page)` if the `data` is not empty.
    /// - `Err(String)` if the `data` is empty.
    ///
    /// # Errors
    /// Returns an error if an empty `Vec<u8>` is passed.
    ///

    pub fn new(is_dirty: bool, data: Vec<u8>) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Page data cannot be empty".to_string());
        }

        Ok(Self {
            _is_dirty: is_dirty,
            _data: data,
        })
    }
}
