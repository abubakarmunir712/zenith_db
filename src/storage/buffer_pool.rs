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

use super::page::Page;



pub struct BufferManager {
    /// It is the actual buffer pool storing pages.
    pool: HashMap<u32, Page>,
    /// This okeps track of the order in which pages were used.
    lru_list: VecDeque<u32>,
    /// It is the maximum number of pages allowed in the buffer pool.
    capacity: usize,
    /// I/O engine for disk operations.
    io_engine: IOEngine,
}

impl BufferManager {
    pub fn new(capacity: usize, io_engine: IOEngine) -> Self {
        Self {
            pool: HashMap::new(),
            lru_list: VecDeque::new(),
            capacity,
            io_engine,
        }
    }

    /// Fetches a page from the buffer pool or loads it from disk if not present.
    pub fn fetch_page(&mut self, page_id: u32) -> &mut Page {
        if self.pool.contains_key(&page_id) {
            self.touch(page_id);
            return self.pool.get_mut(&page_id).unwrap();
        }

        if self.pool.len() >= self.capacity {
            self.evict_page();
        }

        let page_data = self.io_engine.read_page(page_id);
        let page = Page::deserialize(&page_data);
        
        self.pool.insert(page_id, page);
        self.lru_list.push_back(page_id);
        self.pool.get_mut(&page_id).unwrap()
    }

    /// Marks a page as recently used, moving it to the back of the LRU list.
    fn touch(&mut self, page_id: u32) {
        if let Some(pos) = self.lru_list.iter().position(|&id| id == page_id) {
            self.lru_list.remove(pos);
        }
        self.lru_list.push_back(page_id);
    }

    /// Evicts the least recently used (LRU) page from the buffer pool.
    fn evict_page(&mut self) {
        if let Some(oldest_page_id) = self.lru_list.pop_front() {
            if let Some(page) = self.pool.remove(&oldest_page_id) {
                if page.is_dirty {
                    let serialized_data = page.serialize();
                    self.io_engine.update_page(oldest_page_id, &serialized_data);
                }
            }
        }
    }

    /// Flushes all dirty pages to disk.
    pub fn flush(&mut self) {
        for (&page_id, page) in &mut self.pool {
            if page.is_dirty {
                let serialized_data = page.serialize();
                self.io_engine.update_page(page_id, &serialized_data);
            }
        }
    }
}
