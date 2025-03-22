/// A slot entry inside the slot table.
/// Each slot keeps track of a record's position within the page.
/// The slot table allows efficient management of variable-length records.
pub struct Slot {
    /// Byte offset within the page where the record starts.
    /// This helps locate the actual data inside the page.
    pub record_offset: u16,

    /// The size of the record in bytes.
    /// Used to determine how much space the record occupies.
    pub record_size: u16,

    /// Whether the record is deleted (1) or valid (0).
    /// A deleted record's space may be reclaimed for future inserts.
    pub is_deleted: u8,
}
