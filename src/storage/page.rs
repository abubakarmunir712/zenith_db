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

/// Metadata for a database page, stored at the beginning of each page.
/// Helps manage free space, track records, and assist in recovery.
struct PageHeader {
    /// Unique identifier for this page within a file.
    page_id: u32,

    /// Log Sequence Number (LSN) used for Write-Ahead Logging (WAL) and crash recovery.
    lsn: u64,

    /// Offset (in bytes) indicating where the free space starts in the page.
    /// This helps in efficiently finding space for new records.
    free_space_offset: u16,

    /// The total number of records (tuples) currently stored in this page.
    num_of_tuples: u16,

    /// Offset (in bytes) where the slot table begins.
    /// The slot table keeps track of record locations inside the page.
    slot_table_offset: u16,
}

/// A slot entry inside the slot table.
/// Each slot keeps track of a record's position within the page.
struct Slot {
    /// Byte offset within the page where the record starts.
    record_offset: u16,

    /// The size of the record in bytes.
    record_size: u16,

    /// Whether the record is deleted (1) or valid (0).
    is_deleted: u8,
}

/// Represents a database page, containing metadata, data, and a slot table.
/// Pages store records and are managed within the buffer pool.
pub struct Page {
    /// Tracks whether the page has been modified but not written to disk.
    is_dirty: bool,

    /// The actual raw data of the page.
    data: Vec<u8>,

    /// Metadata about the page, such as free space and tuple count.
    page_header: PageHeader,

    /// A table containing offsets to records stored in the data section.
    slot_table: Vec<Slot>,
}

impl Page {
    /// Serializes the Page into a 4KB byte array for storage on disk.
    /// This includes packing the page header, slot table, and data into a single buffer.
    pub fn serialize(&self) -> [u8; 4096] {
        [0; 4096] // Placeholder implementation
    }

    /// Deserializes a 4KB buffer into a Page instance.
    /// Extracts the page header, slot table, and data from raw bytes.
    fn deserialize(buffer: &[u8; 4096]) -> Self {
        Page {
            is_dirty: false,
            data: Vec::new(),
            page_header: PageHeader {
                page_id: 0,
                lsn: 0,
                free_space_offset: 0,
                num_of_tuples: 0,
                slot_table_offset: 0,
            },
            slot_table: Vec::new(),
        }
    }
}
