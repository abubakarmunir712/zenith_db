// buffer_pool.rs - Manages in-memory caching of database pages.
//
// This file implements the Buffer Pool, which stores frequently accessed
// pages in memory to reduce disk I/O and improve performance. It handles
// page fetching, eviction policies, and synchronization
// between in-memory and on-disk data.
//
// The Buffer Pool acts as an intermediary between the storage engine
// and the file system, ensuring efficient data access.
//
use std::collections::{HashMap, VecDeque};
use std::io::Error;

use super::file_io::IOEngine;
use super::page::Page;

pub struct BufferManager {
    /// The actual buffer pool storing pages.
    /// The key is a tuple consisting of:
    /// - `String` -> Database name
    /// - `String` -> File name
    /// - `u32`    -> Page number
    pool: HashMap<(String, String, u32), Page>,
    /// This okeps track of the order in which pages were used.
    lru_list: VecDeque<(String, String, u32)>,
    /// It is the maximum number of pages allowed in the buffer pool.
    capacity: usize,
}

impl BufferManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: HashMap::new(),
            lru_list: VecDeque::new(),
            capacity,
        }
    }

    /// Fetches a page from the buffer pool or loads it from disk if not present.
    pub fn fetch_page(
        &mut self,
        file_name: &str,
        database_name: &str,
        page_id: u32,
    ) -> Result<&mut Page, Error> {
        if self
            .pool
            .contains_key(&(file_name.to_string(), database_name.to_string(), page_id))
        {
            self.touch(&database_name, &file_name, page_id);
            return Ok(self
                .pool
                .get_mut(&(database_name.to_string(), file_name.to_string(), page_id))
                .unwrap());
        }

        // Remove a page from the buffer pool if it has reached its capacity
        if self.pool.len() >= self.capacity {
            self.evict_page()?;
        }

        let page_data = IOEngine::read_page(database_name, file_name, page_id)?;
        let page = Page::deserialize(&page_data);

        self.pool.insert(
            (database_name.to_string(), file_name.to_string(), page_id),
            page,
        );
        self.lru_list.push_back((
            database_name.to_string(),
            file_name.to_string(),
            page_id.clone(),
        ));
        Ok(self
            .pool
            .get_mut(&(database_name.to_string(), file_name.to_string(), page_id))
            .unwrap())
    }

    /// Marks a page as recently used, moving it to the back of the LRU list.
    fn touch(&mut self, database_name: &str, file_name: &str, page_id: u32) {
    
        if let Some(pos) = self
            .lru_list
            .iter()
            .position(|(db, file, id)| db == database_name && file == file_name && *id == page_id)
        {
            self.lru_list.remove(pos);
        }
        self.lru_list
            .push_back((database_name.to_string(), file_name.to_string(), page_id));
    }

    /// Evicts the least recently used (LRU) page from the buffer pool.
    fn evict_page(&mut self) -> Result<(), Error> {
        if let Some(oldest_page_id) = self.lru_list.pop_front() {
            if let Some(page) = self.pool.remove(&oldest_page_id) {
                if page.is_dirty() {
                    let serialized_data = page.serialize();
                    IOEngine::update_page(
                        &oldest_page_id.0,
                        &oldest_page_id.1,
                        oldest_page_id.2,
                        &serialized_data,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Flushes all dirty pages to disk.
    pub fn flush_all(&mut self) -> Result<(), Error> {
        for ((database_name, file_name, page_id), page) in &mut self.pool {
            if page.is_dirty() {
                let serialized_data = page.serialize();
                IOEngine::update_page(database_name, file_name, *page_id, &serialized_data)?;
            }
        }
        Ok(())
    }
}
