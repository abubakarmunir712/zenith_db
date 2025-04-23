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

use super::page_header::PageHeader;
use super::slot::Slot;
use crate::configs::config::Config::{PAGE_HEADER_SIZE, PAGE_SIZE};

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

    /// The number of active references to this page in memory.
    pin_count: u32,
}

impl Page {
    /// Creates a new `Page` with the given `page_id`, `lsn`, `database_name`, and `file_name`.
    ///
    /// # Parameters
    /// - `page_id`: The unique identifier for the page.
    /// - `lsn`: The Log Sequence Number used for Write-Ahead Logging (WAL).
    ///
    /// # Behavior
    /// - The page starts as **dirty** (`is_dirty = true`).
    /// - Initializes an **empty data buffer** (`Vec::new()`).
    /// - Creates a **`PageHeader`** with:
    ///   - `free_space_offset = 20` (free space starts from the 20th byte).
    ///   - `num_of_tuples = 0` (no records initially).
    ///   - `slot_table_offset = 4096` (no slot table yet).
    /// - Initializes an **empty slot table** (`Vec::new()`).
    ///
    /// # Returns
    /// A new `Page` instance with default settings.

    pub fn new(page_id: u32, lsn: u64) -> Self {
        let mut data: Vec<u8> = Vec::with_capacity(4076);
        data.resize(4076, 0);
        Page {
            is_dirty: true,
            data,
            page_header: PageHeader::new(page_id, lsn, PAGE_HEADER_SIZE, 0, PAGE_SIZE as u16),
            slot_table: Vec::new(),
            pin_count: 0,
        }
    }

    /// Serializes the Page into a 4KB byte array for storage on disk.
    /// This includes packing the page header, slot table, and data into a single buffer.
    pub fn serialize(&self, buffer: &mut [u8; PAGE_SIZE as usize]) {
        self.page_header.serialize(buffer);
        self._serialize_data(buffer);
        self._serialize_slot_table(buffer);
    }

    /// Deserializes a 4KB buffer into a Page instance.
    /// Extracts the page header, slot table, and data from raw bytes.
    pub fn deserialize(buffer: &[u8; PAGE_SIZE as usize]) -> Self {
        Page {
            is_dirty: false,
            page_header: PageHeader::deserialize(buffer),
            data: Self::_deserialize_data(buffer),
            slot_table: Self::_deserialize_slot_table(buffer),
            pin_count: 0,
        }
    }

    fn _serialize_data(&self, buffer: &mut [u8; PAGE_SIZE as usize]) {
        if self.page_header.num_of_tuples() == 0 || self.data.len() == 0 {
            return;
        }
        let data_start: usize = 20; // Data segment starts from 20th byte
        let data_end: usize = self.page_header.slot_table_offset() as usize;
        buffer[data_start..data_end].copy_from_slice(&self.data);
    }

    fn _serialize_slot_table(&self, buffer: &mut [u8; PAGE_SIZE as usize]) {
        let mut offset: usize = self.page_header.slot_table_offset() as usize;
        for slot in self.slot_table.iter().rev() {
            slot.serialize(&mut buffer[offset..offset + 8]);
            offset += 8;
        }
    }

    fn _deserialize_data(buffer: &[u8; PAGE_SIZE as usize]) -> Vec<u8> {
        let num_of_tuples = u16::from_le_bytes(buffer[14..16].try_into().unwrap());
        if num_of_tuples == 0 {
            let mut data: Vec<u8> = Vec::with_capacity(4076);
            data.resize(4076, 0);
            return data;
        }
        let data_start: usize = 20;
        let data_end: usize = u16::from_le_bytes(buffer[12..14].try_into().unwrap()) as usize;
        let mut data: Vec<u8> = buffer[data_start..data_end + data_start].to_vec();
        data.resize(4076 - num_of_tuples as usize * 8, 0);
        data
    }

    fn _deserialize_slot_table(buffer: &[u8; PAGE_SIZE as usize]) -> Vec<Slot> {
        let mut slot_table: Vec<Slot> = Vec::new();
        let num_of_tuples = u16::from_le_bytes(buffer[14..16].try_into().unwrap());
        let mut offset = PAGE_SIZE as usize;

        for _ in 0..num_of_tuples {
            offset -= 8;
            let slot = Slot::deserialize(&buffer[offset..offset + 8]);
            slot_table.push(slot);
        }

        slot_table
    }

    // Getter & Setter methods
    ///
    /// It increments the pin count when a page is accessed.
    pub fn pin(&mut self) {
        self.pin_count += 1;
    }

    /// It decrements the pin count when the page is no longer in use.
    pub fn unpin(&mut self) {
        if self.pin_count > 0 {
            self.pin_count -= 1;
        }
    }

    /// It checks if the page is pinned.
    pub fn is_pinned(&self) -> bool {
        self.pin_count > 0
    }

    /// Shrinks the data vector by `change_in_bytes`.
    pub fn decrease_data_len(&mut self, change_in_bytes: usize) -> usize {
        let new_len = self.data.len() - change_in_bytes;
        self.data.resize(new_len, 0);
        new_len
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn page_header(&self) -> &PageHeader {
        &self.page_header
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn set_is_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty
    }

    pub fn slot_table(&self) -> &Vec<Slot> {
        &self.slot_table
    }
    pub fn page_header_as_mut(&mut self) -> &mut PageHeader {
        &mut self.page_header
    }

    pub fn data_as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn slot_table_as_mut(&mut self) -> &mut Vec<Slot> {
        &mut self.slot_table
    }
}
