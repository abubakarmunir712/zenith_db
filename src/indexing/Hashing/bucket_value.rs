/// A `BucketValue` represents a single value in a hash bucket, containing a page number, offset, and deletion flag.
#[derive(Copy, Clone)]
pub struct BucketValue {
    pub page_no: u32,
    pub offset: u16,
    pub is_deleted: u8,
}

impl BucketValue {
    /// Serializes the `BucketValue` into the provided buffer starting from `starting_offset`.
    pub fn serialize(&self, buffer: &mut Vec<u8>, starting_offset: usize) {
        let mut offset = starting_offset;

        // Write page_no (4 bytes)
        buffer[offset..offset + 4].copy_from_slice(&self.page_no.to_le_bytes());
        offset += 4;

        // Write offset (2 bytes)
        buffer[offset..offset + 2].copy_from_slice(&self.offset.to_le_bytes());
        offset += 2;

        // Write is_deleted (1 byte)
        buffer[offset] = self.is_deleted;
    }

    /// Deserializes a `BucketValue` from the given byte buffer starting from `starting_offset`.
    pub fn deserialize(buffer: &Vec<u8>, starting_offset: usize) -> Self {
        let mut offset = starting_offset;

        // Read page_no (4 bytes)
        let page_no = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
        offset += 4;

        // Read offset (2 bytes)
        let offset_val = u16::from_le_bytes(buffer[offset..offset + 2].try_into().unwrap());
        offset += 2;

        // Read is_deleted (1 byte)
        let is_deleted = buffer[offset];

        BucketValue {
            page_no,
            offset: offset_val,
            is_deleted,
        }
    }

    /// Returns the total size of the serialized `BucketValue` in bytes.
    pub fn total_size_in_bytes(&self) -> u16 {
        4 + 2 + 1 // page_no (u32) + offset (u16) + is_deleted (u8)
    }
}
