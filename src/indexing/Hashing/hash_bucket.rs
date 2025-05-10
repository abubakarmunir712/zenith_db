pub const MAX_BUCKET_VALUES: u8 = (((1024 * 2) - (32 + 32 + 8 + 8 + 8)) / (32 + 16 + 8)) as u8; // Floor: 1960 / 56 = 35

use crate::indexing::Hashing::bucket_value::BucketValue;

/// A `HashBucket` represents a bucket in a hash table, containing metadata and a list of values.
pub struct HashBucket {
    pub bucket_no: u32,
    pub is_deleted: u8,
    pub is_overflowed: u8,
    pub next_bucket_pointer: u32, // 0 indicates null; ≥1 indicates overflow bucket number
    pub value_count: u8, // Number of actual values stored
    pub values: Vec<BucketValue>,
    pub next_bucket: Option<Box<HashBucket>>, // No serialization, initialized as None
    pub is_dirty: u8, // No serialization needed
}

impl HashBucket {
    /// Creates a new `HashBucket` with the given bucket number.
    pub fn new(bucket_no: u32) -> Self {
        HashBucket {
            bucket_no,
            is_deleted: 0,
            is_overflowed: 0,
            next_bucket_pointer: 0,
            value_count: 0,
            values: Vec::with_capacity(MAX_BUCKET_VALUES as usize),
            next_bucket: None,
            is_dirty: 0,
        }
    }
}