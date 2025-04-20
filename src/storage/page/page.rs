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
use crate::configs::config::Config::PAGE_SIZE;
use crate::enums::page_errors::PageError;

/// Represents a database page, containing metadata, data, and a slot table.
/// Pages store records and are managed within the buffer pool.
pub struct Page {
    /// Tracks whether the page has been modified but not written to disk.
    is_dirty: bool,

    /// The actual raw data of the page.
    pub data: Vec<u8>,

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
            page_header: PageHeader::new(page_id, lsn, 20, 0, PAGE_SIZE as u16),
            slot_table: Vec::new(),
            pin_count: 0,
        }
    }

    /// Constructor for testing purposes (remove after testing)
    pub fn new_test(page_header: PageHeader, slot_table: Vec<Slot>) -> Self {
        let mut data: Vec<u8> = Vec::with_capacity(4076 - slot_table.len() * 8);
        data.resize(4076 - slot_table.len() * 8, 0);
        Page {
            is_dirty: true,
            data,
            page_header,
            slot_table,
            pin_count: 0,
        }
    }

    /// Serializes the Page into a 4KB byte array for storage on disk.
    /// This includes packing the page header, slot table, and data into a single buffer.
    pub fn serialize(&self) -> [u8; PAGE_SIZE as usize] {
        let mut buffer: [u8; PAGE_SIZE as usize] = [0; PAGE_SIZE as usize];
        self._serialize_page_header(&mut buffer);
        self._serialize_data(&mut buffer);
        self._serialize_slot_table(&mut buffer);
        buffer
    }

    /// Deserializes a 4KB buffer into a Page instance.
    /// Extracts the page header, slot table, and data from raw bytes.
    pub fn deserialize(buffer: &[u8; PAGE_SIZE as usize]) -> Self {
        Page {
            is_dirty: false,
            page_header: Self::_deserialize_page_header(buffer),
            data: Self::_deserialize_data(buffer),
            slot_table: Self::_deserialize_slot_table(buffer),
            pin_count: 0,
        }
    }

    #[rustfmt::skip]
    fn _serialize_page_header(&self, buffer: &mut [u8; PAGE_SIZE as usize]) {
        buffer[0..4].copy_from_slice(&self.page_header.page_id().to_le_bytes());

        buffer[4..12].copy_from_slice(&self.page_header.lsn().to_le_bytes());

        buffer[12..14].copy_from_slice(&self.page_header.free_space_offset().to_le_bytes());

        buffer[14..16].copy_from_slice(&self.page_header.num_of_tuples().to_le_bytes());

        buffer[16..18].copy_from_slice(&self.page_header.slot_table_offset().to_le_bytes());

    }

    fn _serialize_data(&self, buffer: &mut [u8; PAGE_SIZE as usize]) {
        if self.page_header.num_of_tuples() == 0 || self.data.len() == 0 {
            return;
        }
        let data_start: usize = 20; // Data segment starts from 20th byte
        let data_end: usize = self.page_header.slot_table_offset() as usize;
        buffer[data_start..data_end].copy_from_slice(&self.data);
    }

    #[rustfmt::skip]
    fn _serialize_slot_table(&self, buffer: &mut [u8; PAGE_SIZE as usize]) {
        let mut offset: usize = self.page_header.slot_table_offset() as usize;
        let slot_table_len = self.slot_table.len();
        for i in (0..slot_table_len).rev() {
            buffer[offset..offset + 2].copy_from_slice(&self.slot_table[i].record_offset().to_le_bytes());
            offset += 2;

            buffer[offset..offset + 2].copy_from_slice(&self.slot_table[i].record_size().to_le_bytes());
            offset += 2;

            buffer[offset..offset + 1].copy_from_slice(&self.slot_table[i].is_deleted().to_le_bytes());
            offset += 4; // Add extra 3 bytes for padding
        }
    }

    fn _deserialize_page_header(buffer: &[u8; PAGE_SIZE as usize]) -> PageHeader {
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

    fn _deserialize_data(buffer: &[u8; PAGE_SIZE as usize]) -> Vec<u8> {
        let num_of_tuples = u16::from_le_bytes(buffer[14..16].try_into().unwrap());
        if num_of_tuples == 0 {
            let mut data: Vec<u8> = Vec::with_capacity(4076);
            data.resize(4076, 0);
            return data;
        }
        let data_start: usize = 20;
        // Reading free_space_start
        let data_end: usize = u16::from_le_bytes(buffer[12..14].try_into().unwrap()) as usize;
        let mut data: Vec<u8> = buffer[data_start..data_end].to_vec();
        data.reserve(4076 - data.capacity());
        data
    }

    fn _deserialize_slot_table(buffer: &[u8; PAGE_SIZE as usize]) -> Vec<Slot> {
        let mut slot_table: Vec<Slot> = Vec::new();
        let table_offset: usize = u16::from_le_bytes(buffer[16..18].try_into().unwrap()) as usize;
        let num_of_tuples = (4096 - table_offset) / 8;
        let mut offset = 4096;
        for _ in 0..num_of_tuples {
            offset -= 4;
            let is_deleted = u8::from_le_bytes(buffer[offset..offset + 1].try_into().unwrap());
            offset -= 2;
            let record_size = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
            offset -= 2;
            let record_offset = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());

            slot_table.push(Slot::new(record_offset, record_size, is_deleted));
        }

        slot_table
    }

    #[rustfmt::skip]
    /// Returns the total free space in the page (in bytes).
    pub fn free_space_in_bytes(&self) -> u16 {
        let mut free_space = 0;
        let fst = self.free_space_table();
        for fse in fst{
            free_space+=fse.1
        }
        free_space
    }

    /// Returns the largest free space chunk (if any) in the form of (offset, length).
    pub fn max_free_space(&self) -> Option<(u16, u16)> {
        let fst = self.free_space_table();
        let max = fst.into_iter().max_by_key(|x| x.1);
        max
    }

    /// Returns all available free slots in the form `(offset, length)`.
    /// `offset` is the start position, `length` is how many bytes are free from there.
    pub fn free_space_table(&self) -> Vec<(u16, u16)> {
        let mut fst: Vec<(u16, u16)> = Vec::new();
        let free_space =
            self.page_header.slot_table_offset() - self.page_header.free_space_offset();
        if free_space > 0 {
            fst.push((self.page_header.free_space_offset(), free_space));
        }

        let mut prev_slot_end = 0;
        for slot in &self.slot_table {
            if slot.is_deleted() == 1 {
                fst.push((slot.record_offset(), slot.record_size()));
            } else {
                let free_space = slot.record_offset() - prev_slot_end;
                if free_space > 0 {
                    fst.push((prev_slot_end, free_space));
                }
            }
            prev_slot_end = slot.record_offset() + slot.record_size();
        }
        fst
    }

    /// Deletes the top (last) slot in the slot table
    fn _delete_top_slot(&mut self) {
        let mut slot_table_len = self.slot_table.len();

        self.page_header
            .set_slot_table_offset(self.page_header.slot_table_offset() + 8);
        // Add space freed by deleting slot (8 bytes) to data
        self.data.resize(self.data.len() + 8, 0);
        self.slot_table.remove(slot_table_len - 1);
        slot_table_len -= 1;
        if slot_table_len > 0 {
            let free_space_offset = self.slot_table[slot_table_len - 1].record_offset()
                + self.slot_table[slot_table_len - 1].record_size();
            self.page_header.set_free_space_offset(free_space_offset);
            if self.slot_table[slot_table_len - 1].is_deleted() == 1 {
                self._delete_top_slot();
            }
        } else {
            // If slot table is empty set free_space_offset to 20 (default value of empty page)
            self.page_header.set_free_space_offset(20);
        }
    }

    /// Defragments adjacent deleted slots and merges record space
    fn _defragment_slots(&mut self, slot_number: usize) {
        let mut slots_to_del: Vec<usize> = Vec::new();
        let mut offset = self.slot_table[slot_number].record_offset();
        let mut size = self.slot_table[slot_number].record_size();

        if slot_number != 0 {
            let prev_slot = &self.slot_table[slot_number - 1];
            if prev_slot.is_deleted() == 1 {
                slots_to_del.push(slot_number - 1);
                offset = prev_slot.record_offset();
            } else {
                offset = prev_slot.record_offset() + prev_slot.record_size();
            }
        }
        let next_slot = &self.slot_table[slot_number + 1];
        if next_slot.is_deleted() == 1 {
            slots_to_del.push(slot_number + 1);
            size = next_slot.record_offset() + next_slot.record_size() - offset;
        } else {
            size = next_slot.record_offset() - offset;
        }

        self.slot_table[slot_number].set_record_offset(offset);
        self.slot_table[slot_number].set_record_size(size);

        slots_to_del.sort();
        slots_to_del.reverse();
        // Remove all marked deleted slots
        for offset in slots_to_del {
            println!("{}",offset);
            self.slot_table.remove(offset);
        }
    }

    /// Public function to delete a slot by index.
    ///
    /// # Parameters
    /// - `slot_number`: The index of the slot to delete.
    ///
    /// # Returns
    /// - `Result<(), PageError>`: Returns `Ok(())` if the deletion was successful, or `PageError::SlotNotFound` if the index is invalid.
    ///
    /// # Behavior
    /// - If the `slot_number` is 0, it calls `_delete_top_slot()` to handle special logic for the first slot.
    /// - For other slots, it marks the slot as deleted and calls `_defragment_slots()` to compact the data.
    pub fn delete_slot(&mut self, slot_number: usize) -> Result<(), PageError> {
        let slot_table_len = self.slot_table.len();
        if slot_number >= slot_table_len {
            return Err(PageError::SlotNotFound);
        }
        if slot_number == slot_table_len - 1 {
            self._delete_top_slot();
        } else {
            self.slot_table[slot_number].set_is_deleted(1);
            self._defragment_slots(slot_number);
        }
        Ok(())
    }

    /// Adds a new slot to the beginning of the slot table.
    ///
    /// # Parameters
    /// - `record_offset`: The offset where the record starts in the page.
    /// - `record_size`: The size of the record to be stored.
    ///
    /// # Returns
    /// - `usize`: The index at which new slot is inserted in slot table
    pub fn insert_slot(&mut self, record_offset: u16, record_size: u16) -> usize {
        let slot = Slot::new(record_offset, record_size, 0);
        let slot_table_len = self.slot_table.len();
        self.slot_table.push(slot);
        self.data.resize(self.data.len() - 8, 0);
        slot_table_len
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
