use serde::de::value;

use crate::storage::{
    catalog::maps::column_map::{self, ColumnMap},
    page::{page::Page, page_manager::PageManager},
};

use super::record::Record;

pub struct RecordManager;

impl RecordManager {
    // Returns the offset where record is inserted
    pub fn insert_record(record: &[u8], page: &mut Page) -> Option<(u16, u16)> {
        let record_len = record.len();
        let free_offsets = PageManager::free_space_table(&page);
        for free_offset in free_offsets {
            if free_offset.1 as usize > record_len {
                page.data_as_mut()[free_offset.0 as usize..free_offset.0 as usize + record_len]
                    .copy_from_slice(record);
                PageManager::insert_slot(page, free_offset.0, record_len as u16);
                return Some((free_offset.0, record_len as u16));
            }
        }
        return None;
    }

    pub fn read_records(page: &Page, column_map: &ColumnMap) -> Vec<Record> {
        let mut records: Vec<Record> = Vec::new();
        for slot in page.slot_table() {
            if slot.is_deleted() == 0 {
                let bytes = &page.data()[slot.record_offset() as usize
                    ..slot.record_size() as usize + slot.record_offset() as usize];
                let record = Record::deserialize(bytes, column_map);
                records.push(record);
            }
        }
        records
    }

    pub fn get_record_by_offset(page: &Page, offset: u16, column_map: &ColumnMap) -> Record {
        let slot = PageManager::get_slot_by_offset(page, offset).unwrap();
        let bytes = &page.data()[slot.record_offset() as usize
            ..slot.record_size() as usize + slot.record_offset() as usize];
        let record = Record::deserialize(bytes, column_map);
        record
    }

    pub fn get_records_by_value(
        page: &Page,
        value: &str,
        column_name: &str,
        column_map: &ColumnMap,
    ) -> Vec<Record> {
        let mut records: Vec<Record> = Vec::new();
        let column_no = column_map
            .ord_map()
            .iter()
            .position(|x| x == column_name)
            .unwrap();
        for slot in page.slot_table() {
            if slot.is_deleted() == 0 {
                let bytes = &page.data()[slot.record_offset() as usize
                    ..slot.record_size() as usize + slot.record_offset() as usize];
                let record = Record::deserialize(bytes, column_map);
                let col = record.columns()[column_no].to_string();
                if col == value {
                    records.push(record);
                }
            }
        }
        records
    }

    pub fn delete_record_by_value(
        page: &mut Page,
        value: &str,
        column_name: &str,
        column_map: &ColumnMap,
    ) {
        let column_no = column_map
            .ord_map()
            .iter()
            .position(|x| x == column_name)
            .unwrap();
        let mut slots = Vec::new();
        for slot in page.slot_table() {
            if slot.is_deleted() == 0 {
                let bytes = &page.data()[slot.record_offset() as usize
                    ..slot.record_size() as usize + slot.record_offset() as usize];
                let record = Record::deserialize(bytes, column_map);
                let col = record.columns()[column_no].to_string();
                if col == value {
                    slots.push(slot.record_offset());
                }
            }
        }
        for slot in slots {
            PageManager::mark_slot_as_deleted(page, slot);
        }
    }

    pub fn get_column_value(record: &Record, column_name: &str, column_map: &ColumnMap) -> String {
        let column_no = column_map
            .ord_map()
            .iter()
            .position(|x| x == column_name)
            .unwrap();
        let col = record.columns()[column_no].to_string();
        col
    }
}
