use std::sync::Arc;

use super::bucket_value;
use super::hashing_buffer::HashingBuffer;
use crate::configs::config::Config::INDEX_PAGE_SIZE;
use crate::enums::types::page_types::PageType;
use crate::indexing::Hashing::bucket_value::BucketValue;
use crate::indexing::Hashing::hash_bucket::HashBucket;
use crate::storage::buffer::index_buffer::IndexBuffer;
use crate::storage::buffer::page_buffer::PageBuffer;
use crate::storage::catalog::maps::column_map::ColumnMap;
use crate::storage::io::file_io::IOEngine;
use crate::storage::page::page::Page;
use crate::storage::record::record::Record;
use crate::storage::record::record_manager::RecordManager;
pub const MAX_BUCKET_VALUES: u8 = (((1024 * 2) - (32 + 32 + 8 + 8 + 8)) / (32 + 16 + 8)) as u8; // Floor: 1960 / 56 = 35

/// A `HashBucketSerializer` handles serialization and deserialization of `HashBucket` instances.
pub struct HashBucketManager;

impl HashBucketManager {
    /// Serializes the `HashBucket` into the provided buffer starting from `starting_offset`.
    pub fn serialize(bucket: &HashBucket, buffer: &mut [u8], starting_offset: usize) {
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
        bucket.next_bucket_pointer =
            u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
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

    // pub fn dddadd_value(key:&str,db_name:&str,table_column:&str,is_overflow:u8,buffer: &IndexBuffer,bucket: &mut HashBucket, value: &BucketValue) -> Result<(), String>{

    //     let index = HashBucketManager::murmur_hash(key);
    //     let page = buffer.get_page(db_name, table_column, is_overflow, index)?;
    //     let mut bucket = page.write().map_err(|e| e.to_string())?;

    //     let mut curren_bucket: Option<&mut HashBucket> = Some(&mut bucket); // Get By Buffer later

    //     let value = value.clone();

    //     let mut current_bucket = bucket;

    //     while let Some(b) = Some(current_bucket) {
    //         //first we try to place in deleted slot
    //         for i in 0..b.value_count as usize {
    //             if b.values[i].is_deleted != 0 {
    //                 b.values[i] = value;
    //                 b.is_dirty = 1;
    //                 Ok(())
    //             }
    //         }

    //         // second, we try to place at end
    //         if b.value_count < MAX_BUCKET_VALUES {
    //             if (b.value_count as usize) < b.values.len() {
    //                 b.values[b.value_count as usize] = value;
    //             } else {
    //                 b.values.push(value);
    //             }
    //             b.value_count += 1;
    //             b.is_dirty = 1;
    //             Ok(())
    //         }

    //         // third, we go to next bucket if exists
    //         if let Some(ref mut next) = b.next_bucket {
    //             current_bucket = next.as_mut();
    //         } else {
    //             // fourth, wo next_bucket, create new
    //             //LOGIC TO BE CHANGED
    //             //IT IS ADD just as placeholder
    //             let mut new_bucket = HashBucket::new(0); // change `0` to desired bucket_no
    //             new_bucket.values.push(value);
    //             new_bucket.value_count = 1;
    //             new_bucket.is_dirty = 1;
    //             b.next_bucket = Some(Box::new(new_bucket));
    //             Ok(())
    //         }

    //     }
    //     Ok(())
    // }
    pub fn add_value(
        key: &str,
        value: &BucketValue,
        db_name: &str,
        table_column: &str,
        is_overflow: u8,
        index_buffer: &IndexBuffer,
    ) -> Result<(), String> {
        let index = HashBucketManager::murmur_hash(key);
        let page = index_buffer.get_page(db_name, table_column, is_overflow, index)?;
        let mut guard = page.write().map_err(|e| e.to_string())?;
        let mut current_bucket: &mut HashBucket = &mut *guard;

        let value = value.clone();

        loop {
            // first we try to place in deleted slot
            for i in 0..current_bucket.value_count as usize {
                if current_bucket.values[i].is_deleted != 0 {
                    current_bucket.values[i] = value;
                    current_bucket.is_dirty = 1;
                    return Ok(());
                }
            }

            // second, we try to place at end
            if current_bucket.value_count < MAX_BUCKET_VALUES {
                if (current_bucket.value_count as usize) < current_bucket.values.len() {
                    current_bucket.values[current_bucket.value_count as usize] = value;
                } else {
                    current_bucket.values.push(value);
                }
                current_bucket.value_count += 1;
                current_bucket.is_dirty = 1;
                return Ok(());
            }

            // third, we go to next bucket if exists
            if let Some(ref mut next) = current_bucket.next_bucket {
                current_bucket = next.as_mut();
            } else {
                // fourth, wo next_bucket, create new
                let total_pages =
                    IOEngine::calculate_total_pages(db_name, table_column, PageType::OverflowPage)?
                        + 1;
                let mut new_bucket = HashBucket::new(total_pages); // change `0` to desired bucket_no

                new_bucket.values.push(value);
                new_bucket.value_count = 1;
                new_bucket.is_dirty = 1;
                let mut buffer = [0u8; INDEX_PAGE_SIZE as usize];
                HashBucketManager::serialize(&new_bucket, &mut buffer, 0);
                current_bucket.next_bucket = Some(Box::new(new_bucket));
                IOEngine::append_page(db_name, table_column, &buffer, PageType::OverflowPage)?;
                index_buffer._get_page(db_name, table_column, 1, total_pages, true)?;
                return Ok(());
            }
        }
    }

    // pub fn add_value_to_bucket(bucket: &mut HashBucket, value: &BucketValue) {
    //     // Assume BucketValue implements Clone for copying the value
    //     let value = value.clone();

    //     // Check for a deleted slot within the used portion (up to value_count)
    //     for i in 0..bucket.value_count as usize {
    //         if bucket.values[i].is_deleted != 0 {
    //             // Found a deleted slot, replace it
    //             bucket.values[i] = value;
    //             bucket.is_dirty = 1; // Mark bucket as dirty
    //             return;
    //         }
    //     }
    //     if bucket.value_count < MAX_BUCKET_VALUES {
    //         // No deleted slot found, append to the end (within value_count)
    //         if (bucket.value_count as usize) < bucket.values.len() {
    //             // Replace the value at value_count
    //             bucket.values[bucket.value_count as usize] = value;
    //         } else {
    //             // Append to the vector
    //             bucket.values.push(value);
    //         }
    //         bucket.value_count += 1;
    //         bucket.is_dirty = 1; // Mark bucket as dirty
    //     } else {
    //         if let Some(next) = &mut bucket.next_bucket {
    //             //If Not None
    //         } else {
    //             //next_bucket is None
    //     }
    // }

    pub fn get_values(
        key: &str,
        db_name: &str,
        table_id: u32,
        column_name: &str,
        column_map: &ColumnMap,
        pgbuffer: &Arc<PageBuffer>,
        buffer: &IndexBuffer,
    ) -> Result<Vec<Record>, String> {
        let mut records: Vec<Record> = Vec::new();
        let index = HashBucketManager::murmur_hash(key);
        let is_overflow = 0;
        println!("-->{column_name}");
        let column_id = column_map.get_column(column_name).unwrap().oid();
        let combined_string = format!("{}_{}", table_id, column_id.to_string());
        let table_column: &str = &combined_string;

        // let mut bucket = HashBucket::new(0);
        // Get the page and mark it dirty
        let page = buffer.get_page(db_name, table_column, is_overflow, index)?;
        let mut bucket = page.write().map_err(|e| e.to_string())?;

        let mut bucket_ref: Option<&mut HashBucket> = Some(&mut bucket); // Get By Buffer later

        while let Some(b_ref) = bucket_ref {
            for i in 0..b_ref.value_count as usize {
                let bucket_value = b_ref.values[i];
                if bucket_value.is_deleted == 0 {
                    let page_arc = pgbuffer
                        .get_page(db_name, &table_id.to_string(), bucket_value.page_no, false)
                        .unwrap();
                    let page_read = page_arc.read().unwrap();
                    let page: &Page = &*page_read;

                    // let page:&Page= pgBuffer::get_page(db_name,table_id.as_str() , bucket_value.page_no, 0).unwrap();
                    let record =
                        RecordManager::get_record_by_offset(page, bucket_value.offset, column_map);
                    let temporary_value =
                        RecordManager::get_column_value(&record, column_name, column_map);

                    if temporary_value == key {
                        records.push(record);
                    }
                }
                // process bucket_value here
            }

            bucket_ref = b_ref.next_bucket.as_deref_mut();
        }

        Ok(records)
    }
    pub fn delete_value(
        buffer: &IndexBuffer,
        db_name: &str,
        table_column: &str,
        offset: u16,
        key: &str,
    ) -> Result<bool, String> {
        let current_page_no = HashBucketManager::murmur_hash(key);
        let page_no = current_page_no;
        let is_overflow = 0;

        // Get the first page and lock it for write access
        let page = buffer.get_page(db_name, table_column, is_overflow, current_page_no)?;
        let mut bucket = page.write().map_err(|e| e.to_string())?;

        let mut bucket_ref: Option<&mut HashBucket> = Some(&mut bucket); // Get the current bucket

        while let Some(b_ref) = bucket_ref {
            // Iterate through values in the current bucket
            for i in 0..b_ref.value_count as usize {
                let value = &mut b_ref.values[i];

                // Check for the value we need to delete
                if value.page_no == page_no && value.offset == offset && value.is_deleted == 0 {
                    value.is_deleted = 1; // Mark the value as deleted
                    b_ref.is_dirty = 1; // Mark the bucket as dirty
                    return Ok(true); // Return true since we successfully deleted the record
                }
            }

            // Move to the next bucket if it exists
            bucket_ref = b_ref.next_bucket.as_deref_mut();
        }

        // If no matching value is found, return false
        Ok(false)
    }

    // pub fn delete_value(bucket: &mut HashBucket, page_no: u32, offset: u16) {
    // let mut current_bucket = bucket;

    // while let Some(b) = Some(current_bucket) {
    //     for i in 0..b.value_count as usize {
    //         let val = &mut b.values[i];
    //         if val.page_no == page_no && val.offset == offset && val.is_deleted == 0 {
    //             val.is_deleted = 1;
    //             b.is_dirty = 1;
    //             return;
    //         }
    //     }

    //     // Move to the next bucket if exists
    //     if let Some(ref mut next) = b.next_bucket {
    //         current_bucket = next.as_mut();
    //     } else {
    //         // No match found and no next bucket — exit
    //         return;
    //     }
    // }

    pub fn murmur_hash(key: &str) -> u32 {
        let seed: u32 = 0x9747b28c; // Fixed seed
        let data = key.as_bytes();
        let len = data.len() as u32;
        let mut hash = seed;

        let c1: u32 = 0xcc9e2d51;
        let c2: u32 = 0x1b873593;
        let r1 = 15;
        let r2 = 13;
        let m: u32 = 5;
        let n: u32 = 0xe6546b64;

        let mut i = 0;
        while i + 4 <= data.len() {
            let k = u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);

            let mut k = k.wrapping_mul(c1);
            k = k.rotate_left(r1);
            k = k.wrapping_mul(c2);

            hash ^= k;
            hash = hash.rotate_left(r2);
            hash = hash.wrapping_mul(m).wrapping_add(n);

            i += 4;
        }

        // Tail
        let mut k1: u32 = 0;
        let rem = data.len() & 3;
        if rem == 3 {
            k1 ^= (data[i + 2] as u32) << 16;
        }
        if rem >= 2 {
            k1 ^= (data[i + 1] as u32) << 8;
        }
        if rem >= 1 {
            k1 ^= data[i] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(r1);
            k1 = k1.wrapping_mul(c2);
            hash ^= k1;
        }

        // Finalization
        hash ^= len;
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(0x85ebca6b);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(0xc2b2ae35);
        hash ^= hash >> 16;

        (hash % 10000) + 1
    }

    pub fn create_index(
        db_name: &str,
        table_id: u32,
        column_name: &str,
        column_map: &ColumnMap,
        buffer: &IndexBuffer,
    ) -> Result<(), String> {
        let column_id = column_map.get_column(column_name).unwrap().oid();
        IOEngine::create_index(db_name, &format!("{}_{}", table_id, column_id)).unwrap();

        for i in 0..10000 {
            let bucket = HashBucket::new(i);
            let mut buffer = [0u8; INDEX_PAGE_SIZE as usize];
            HashBucketManager::serialize(&bucket, &mut buffer, 0);
            IOEngine::append_page(
                db_name,
                &format!("{}_{}", table_id, column_id),
                &mut buffer,
                PageType::IndexPage,
            )?;
        }
        Ok(())
    }

    pub fn does_key_exists(
        key: &str,
        db_name: &str,
        table_id: u32,
        column_name: &str,
        column_map: &ColumnMap,
        pgbuffer: &Arc<PageBuffer>,
        buffer: &Arc<IndexBuffer>,
    ) -> Result<bool, String> {
        let records = Self::get_values(
            key,
            db_name,
            table_id,
            column_name,
            column_map,
            pgbuffer,
            buffer,
        )?;
        if records.len() == 0 {
            return Ok(false);
        }
        Ok(true)
    }
}
