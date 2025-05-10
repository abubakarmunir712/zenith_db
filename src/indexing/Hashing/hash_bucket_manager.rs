use crate::indexing::Hashing::bucket_value::BucketValue;
use crate::indexing::Hashing::hash_bucket::HashBucket;
/// A `HashBucketSerializer` handles serialization and deserialization of `HashBucket` instances.
pub struct HashBucketManager;

impl HashBucketManager {
    /// Serializes the `HashBucket` into the provided buffer starting from `starting_offset`.
    pub fn serialize(bucket: &HashBucket, buffer: &mut Vec<u8>, starting_offset: usize) {
        let mut offset = starting_offset;

        // Write bucket_no (4 bytes)
        buffer[offset..offset + 4].copy_from_slice(&bucket.bucket_no.to_le_bytes());
        offset += 4;

        // Write is_deleted (1 byte)
        buffer[offset] = bucket.is_deleted;
        offset += 1;

        // Write is_overflowed (1 byte)
        buffer[offset] = bucket.is_overflowed;
        offset += 1;

        // Write next_bucket_pointer (4 bytes)
        buffer[offset..offset + 4].copy_from_slice(&bucket.next_bucket_pointer.to_le_bytes());
        offset += 4;

        // Write value_count (1 byte)
        buffer[offset] = bucket.value_count;
        offset += 1;

        // Serialize actual values up to value_count
        for value in bucket.values.iter().take(bucket.value_count as usize) {
            value.serialize(buffer, offset);
            offset += value.total_size_in_bytes() as usize;
        }
    }

    /// Deserializes a `HashBucket` from the given byte buffer starting from `starting_offset`.
    pub fn deserialize(buffer: &Vec<u8>, starting_offset: usize) -> HashBucket {
        let mut offset = starting_offset;

        // Read bucket_no (4 bytes)
        let bucket_no = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
        offset += 4;

        // Create a new HashBucket using the `new` function
        let mut bucket = HashBucket::new(bucket_no);

        // Read is_deleted (1 byte)
        bucket.is_deleted = buffer[offset];
        offset += 1;

        // Read is_overflowed (1 byte)
        bucket.is_overflowed = buffer[offset];
        offset += 1;

        // Read next_bucket_pointer (4 bytes)
        bucket.next_bucket_pointer = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
        offset += 4;

        // Read value_count (1 byte)
        bucket.value_count = buffer[offset];
        offset += 1;

        // Read values up to value_count
        bucket.values.clear(); // Clear the pre-allocated vector
        for _ in 0..bucket.value_count {
            let value = BucketValue::deserialize(buffer, offset);
            offset += value.total_size_in_bytes() as usize;
            bucket.values.push(value);
        }
        bucket
    }
    /// Adds a `BucketValue` to the `HashBucket`, reusing a deleted slot if available or appending to the end.
    pub fn add_value_to_bucket(bucket: &mut HashBucket, value: &BucketValue) {
        // Assume BucketValue implements Clone for copying the value
        let value = value.clone();

        // Check for a deleted slot within the used portion (up to value_count)
        for i in 0..bucket.value_count as usize {
            if bucket.values[i].is_deleted != 0 {
                // Found a deleted slot, replace it
                bucket.values[i] = value;
                bucket.is_dirty = 1; // Mark bucket as dirty
                return;
            }
        }

        // No deleted slot found, append to the end (within value_count)
        if (bucket.value_count as usize) < bucket.values.len() {
            // Replace the value at value_count
            bucket.values[bucket.value_count as usize] = value;
        } else {
            // Append to the vector
            bucket.values.push(value);
        }
        bucket.value_count += 1;
        bucket.is_dirty = 1; // Mark bucket as dirty
    }
}