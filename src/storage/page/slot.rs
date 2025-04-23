/// A slot entry inside the slot table.
/// Each slot keeps track of a record's position within the page.
/// The slot table allows efficient management of variable-length records.
///
use crate::configs::config::Config::PAGE_SIZE;

pub struct Slot {
    /// Byte offset within the page where the record starts.
    /// This helps locate the actual data inside the page.
    record_offset: u16,

    /// The size of the record in bytes.
    /// Used to determine how much space the record occupies.
    record_size: u16,

    /// Whether the record is deleted (1) or valid (0).
    /// A deleted record's space may be reclaimed for future inserts.
    is_deleted: u8,

    /// Total size of the slot in bytes.
    /// This is set at the time of slot creation and remains unchanged afterwards.
    total_size: u16,
}

impl Slot {
    pub fn new(record_offset: u16, record_size: u16, is_deleted: u8, total_size: u16) -> Self {
        Slot {
            record_offset,
            record_size,
            is_deleted,
            total_size,
        }
    }

    pub fn serialize(&self, buffer: &mut [u8]) {
        buffer[0..2].copy_from_slice(&self.record_offset.to_le_bytes());
        buffer[2..4].copy_from_slice(&self.record_size.to_le_bytes());
        buffer[4..5].copy_from_slice(&self.is_deleted.to_le_bytes());
        buffer[5..7].copy_from_slice(&self.total_size.to_le_bytes());
        buffer[7] = 0;
    }

    pub fn deserialize(buffer: &[u8]) -> Self {
        let record_offset = u16::from_le_bytes(buffer[0..2].try_into().unwrap());
        let record_size = u16::from_le_bytes(buffer[2..4].try_into().unwrap());
        let is_deleted = buffer[4];
        let total_size = u16::from_le_bytes(buffer[5..7].try_into().unwrap());

        Slot::new(record_offset, record_size, is_deleted, total_size)
    }

    /// Getter & setter functions
    pub fn record_offset(&self) -> u16 {
        self.record_offset
    }

    pub fn record_size(&self) -> u16 {
        self.record_size
    }

    pub fn is_deleted(&self) -> u8 {
        self.is_deleted
    }

    pub fn total_size(&self) -> u16 {
        self.total_size
    }

    pub fn set_record_size(&mut self, record_size: u16) {
        self.record_size = record_size;
    }

    pub fn set_record_offset(&mut self, record_offset: u16) {
        self.record_offset = record_offset;
    }

    pub fn set_is_deleted(&mut self, is_deleted: u8) {
        self.is_deleted = is_deleted;
    }
}
