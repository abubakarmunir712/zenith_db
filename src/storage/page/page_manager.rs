use crate::configs::config::Config::PAGE_HEADER_SIZE;

/// `PageManager` handles all operations on `Page` objects like inserting slots, marking them deleted,
/// and calculating available free space inside a page.
use super::{page::Page, slot::Slot};

pub struct PageManager {}

impl PageManager {
    #[rustfmt::skip]
    /// Inserts a new slot or updates an existing deleted slot with the given record offset and size.
    /// - If a slot with the same offset exists, it reuses it.
    /// - Otherwise, adds a new slot, updates page header, and adjusts free space + data length.
    pub fn insert_slot(page: &mut Page, record_offset: u16, record_size: u16) {
        let slot_table: &mut Vec<Slot> = page.slot_table_as_mut();
        for i in 0..slot_table.len() {
            if slot_table[i].record_offset() == record_offset {
                slot_table[i].set_record_offset(record_offset);
                slot_table[i].set_record_size(record_size);
                slot_table[i].set_is_deleted(0);
                return;
            }
        }
        let slot = Slot::new(record_offset, record_size, 0, record_size);
        slot_table.push(slot);
        let page_header = page.page_header_as_mut();
        page_header.set_slot_table_offset(page_header.slot_table_offset() - 8);
        page_header.inc_num_of_tuples(1);
        page.decrease_data_len(8);
        page.page_header_as_mut().set_free_space_offset(PAGE_HEADER_SIZE+record_offset + record_size);
    }

    /// Marks the slot at the given offset as deleted (does not remove it).
    /// - Used for logical deletion to support slot reuse later.
    pub fn mark_slot_as_deleted(page: &mut Page, offset: u16) {
        let slot_table = page.slot_table_as_mut();
        for i in 0..slot_table.len() {
            if slot_table[i].record_offset() == offset {
                slot_table[i].set_is_deleted(1);
            }
        }
    }

    /// Returns a list of deleted slots as a `Vec<(u16, u16)>`.
    /// - Each tuple is: (record_offset, total_size of the deleted record).
    /// - Useful for finding reusable space in the page.
    pub fn get_deleted_slots_offsets(page: &Page) -> Vec<(u16, u16)> {
        let mut slot_offsets = Vec::new();
        for slot in page.slot_table() {
            if slot.is_deleted() == 1 {
                slot_offsets.push((slot.record_offset(), slot.total_size()));
            }
        }
        slot_offsets
    }

    /// Returns the page's available free space as a `Vec<(u16, u16)>`.
    /// - Each tuple is: (offset, size of available space).
    /// - Includes both deleted slots and actual contiguous free space between
    ///   the free_space_offset and the slot_table_offset.
    pub fn free_space_table(page: &Page) -> Vec<(u16, u16)> {
        let mut slot_offsets = Self::get_deleted_slots_offsets(page);
        let page_header = page.page_header();
        if page_header.free_space_offset() < page_header.slot_table_offset() {
            slot_offsets.push((
                page_header.free_space_offset() - PAGE_HEADER_SIZE,
                page_header.slot_table_offset() - page_header.free_space_offset(),
            ))
        }
        slot_offsets
    }

    pub fn get_slot_by_offset(page: &Page, offset: u16) -> Option<&Slot> {
        for slot in page.slot_table() {
            if slot.record_offset() == offset {
                return Some(slot);
            }
        }
        return None;
    }
}
