/// Metadata for a database page, stored at the beginning of each page.
/// Helps manage free space, track records, and assist in recovery.
pub struct PageHeader {
    /// Unique identifier for this page within a file.
    page_id: u32,

    /// Log Sequence Number (LSN) used for Write-Ahead Logging (WAL) and crash recovery.
    lsn: u64,

    /// Offset (in bytes) indicating where the free space starts in the page.
    /// This helps in efficiently finding space for new records.
    free_space_offset: u16,

    /// The total number of valid records (tuples) currently stored in this page.
    num_of_tuples: u16,

    /// Offset (in bytes) where the slot table begins.
    /// The slot table keeps track of record locations inside the page.
    slot_table_offset: u16,
}

impl PageHeader {
    /// Creates a new `PageHeader` with the given metadata.
    ///
    /// # Parameters
    /// - `page_id`: The unique identifier for this page.
    /// - `lsn`: Log Sequence Number for WAL and recovery.
    /// - `free_space_offset`: The starting byte offset of free space in the page.
    /// - `num_of_tuples`: The number of records (tuples) currently stored.
    /// - `slot_table_offset`: The starting byte offset of the slot table.
    ///
    /// # Returns
    /// A new `PageHeader` instance with the specified values.
    pub fn new(
        page_id: u32,
        lsn: u64,
        free_space_offset: u16,
        num_of_tuples: u16,
        slot_table_offset: u16,
    ) -> Self {
        PageHeader {
            page_id,
            lsn,
            free_space_offset,
            num_of_tuples,
            slot_table_offset,
        }
    }

    /// Getter & setter methods
    ///

    pub fn page_id(&self) -> u32 {
        self.page_id
    }

    pub fn lsn(&self) -> u64 {
        self.lsn
    }

    pub fn free_space_offset(&self) -> u16 {
        self.free_space_offset
    }

    pub fn num_of_tuples(&self) -> u16 {
        self.num_of_tuples
    }

    pub fn slot_table_offset(&self) -> u16 {
        self.slot_table_offset
    }

    pub fn set_free_space_offset(&mut self, free_space_offset: u16) {
        self.free_space_offset = free_space_offset;
    }

    pub fn set_slot_table_offset(&mut self, slot_table_offset: u16) {
        self.slot_table_offset = slot_table_offset;
    }
}
