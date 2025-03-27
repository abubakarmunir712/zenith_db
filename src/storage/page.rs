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
        Page {
            is_dirty: true,
            data: Vec::new(),
            page_header: PageHeader::new(page_id, lsn, 20, 0, 4096),
            slot_table: Vec::new(),
            pin_count: 0,
        }
    }

    /// Serializes the Page into a 4KB byte array for storage on disk.
    /// This includes packing the page header, slot table, and data into a single buffer.
    pub fn serialize(&self) -> [u8; 4096] {
        let mut buffer: [u8; 4096] = [0; 4096];
        self._serialize_page_header(&mut buffer);
        self._serialize_data(&mut buffer);
        self._serialize_slot_table(&mut buffer);
        buffer
    }

    /// Deserializes a 4KB buffer into a Page instance.
    /// Extracts the page header, slot table, and data from raw bytes.
    pub fn deserialize(buffer: &[u8; 4096]) -> Self {
        Page {
            is_dirty: false,
            data: Self::_deserialize_data(buffer),
            page_header: Self::_deserialize_page_header(buffer),
            slot_table: Self::_deserialize_slot_table(buffer),
            pin_count: 0,
        }
    }

    #[rustfmt::skip]
    fn _serialize_page_header(&self, buffer: &mut [u8; 4096]) {
        buffer[0..4].copy_from_slice(&self.page_header.page_id().to_le_bytes());

        buffer[4..12].copy_from_slice(&self.page_header.lsn().to_le_bytes());

        buffer[12..14].copy_from_slice(&self.page_header.free_space_offset().to_le_bytes());

        buffer[14..16].copy_from_slice(&self.page_header.num_of_tuples().to_le_bytes());

        buffer[16..18].copy_from_slice(&self.page_header.slot_table_offset().to_le_bytes());

    }

    fn _serialize_data(&self, buffer: &mut [u8; 4096]) {
        if self.page_header.num_of_tuples() == 0 || self.data.len() == 0 {
            return;
        }
        let data_start: usize = 20; // Data segment starts from 20th byte
        let data_end: usize = self.page_header.free_space_offset() as usize;
        buffer[data_start..data_end].copy_from_slice(&self.data);
    }

    fn _serialize_slot_table(&self, buffer: &mut [u8; 4096]) {
        let mut offset: usize = self.page_header.slot_table_offset() as usize;
        for slot in &self.slot_table {
            buffer[offset..offset + 2].copy_from_slice(&slot.record_offset().to_le_bytes());
            offset += 2;

            buffer[offset..offset + 2].copy_from_slice(&slot.record_size().to_le_bytes());
            offset += 2;

            buffer[offset..offset + 1].copy_from_slice(&slot.is_deleted().to_le_bytes());
            offset += 4; // Add extra 3 bytes for padding
        }
    }

    fn _deserialize_page_header(buffer: &[u8; 4096]) -> PageHeader {
        let page_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let lsn = u64::from_le_bytes(buffer[4..12].try_into().unwrap());
        let free_space_offset = u16::from_le_bytes(buffer[12..14].try_into().unwrap());
        let num_of_tuples = u16::from_le_bytes(buffer[14..16].try_into().unwrap());
        let slot_table_offset = u16::from_le_bytes(buffer[16..18].try_into().unwrap());

        PageHeader::new(
            page_id,
            lsn,
            free_space_offset,
            num_of_tuples,
            slot_table_offset,
        )
    }

    fn _deserialize_data(buffer: &[u8; 4096]) -> Vec<u8> {
        let num_of_tuples = u16::from_le_bytes(buffer[14..16].try_into().unwrap());
        if num_of_tuples == 0 {
            return Vec::new();
        }
        let data_start: usize = 20;
        // Reading free_space_start
        let data_end: usize = u16::from_le_bytes(buffer[12..14].try_into().unwrap()) as usize;
        let data: Vec<u8> = buffer[data_start..data_end].to_vec();
        data
    }

    fn _deserialize_slot_table(buffer: &[u8; 4096]) -> Vec<Slot> {
        let mut slot_table: Vec<Slot> = Vec::new();
        let mut offset: usize = u16::from_le_bytes(buffer[16..18].try_into().unwrap()) as usize;
        let num_of_tuples = u16::from_le_bytes(buffer[14..16].try_into().unwrap());

        for _ in 0..num_of_tuples {
            let record_offset = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
            offset += 2;
            let record_size = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
            offset += 2;
            let is_deleted = u8::from_le_bytes(buffer[offset..offset + 1].try_into().unwrap());
            offset += 4;

            slot_table.push(Slot::new(record_offset, record_size, is_deleted));
        }

        slot_table
    }

    /// Getter & Setter methods
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
}
