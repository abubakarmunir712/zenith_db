/// A slot entry inside the slot table.
/// Each slot keeps track of a record's position within the page.
/// The slot table allows efficient management of variable-length records.
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
}

impl Slot {
    pub fn new(record_offset: u16, record_size: u16, is_deleted: u8) -> Self {
        Slot {
            record_offset,
            record_size,
            is_deleted,
        }
    }

    /// Getter functions
    pub fn record_offset(&self) -> u16 {
        self.record_offset
    }

    pub fn record_size(&self) -> u16 {
        self.record_size
    }

    pub fn is_deleted(&self) -> u8 {
        self.is_deleted
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
