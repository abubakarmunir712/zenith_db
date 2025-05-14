use crate::configs::config::Config::{IDX_BUF_CAP, INDEX_PAGE_SIZE};
use crate::enums::types::page_types::PageType;
use crate::indexing::Hashing::hash_bucket::HashBucket;
use crate::indexing::Hashing::hash_bucket_manager::HashBucketManager;
use crate::storage::io::file_io::IOEngine;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// PageBuffer handles in-memory caching of pages with LRU eviction and dirty page tracking.
pub struct IndexBuffer {
    // Main pool of catalog pages in memory. Key: (db_name, table_column, page_number, is_overflow)
    pool: Arc<RwLock<HashMap<(String, String, u32, u8), Arc<RwLock<HashBucket>>>>>,
    // Tracks order of page usage for eviction (LRU).
    lru_list: Arc<RwLock<VecDeque<(String, String, u32, u8)>>>,
    // Keeps track of dirty (modified) pages.
    dirty_pages: Arc<RwLock<HashSet<(String, String, u32, u8)>>>,
}

impl IndexBuffer {
    /// Creates a new catalog buffer with empty cache.
    pub fn new() -> Arc<IndexBuffer> {
        Arc::new(IndexBuffer {
            pool: Arc::new(RwLock::new(HashMap::with_capacity(IDX_BUF_CAP as usize))),
            lru_list: Arc::new(RwLock::new(VecDeque::new())),
            dirty_pages: Arc::new(RwLock::new(HashSet::new())),
        })
    }

    /// Fetch a page from buffer or disk.
    ///
    /// If `mark_dirty` is true, marks it dirty (use this if you're gonna *mutate* the page).
    /// Loads from disk if not found in memory.
    pub fn get_page(
        &self,
        db_name: &str,
        table_column: &str,
        is_overflow: u8,
        page_number: u32,
        mark_dirty: bool,
    ) -> Result<Arc<RwLock<HashBucket>>, String> {
        let key = (
            db_name.to_string(),
            table_column.to_string(),
            page_number,
            is_overflow,
        );
        if mark_dirty {
            let mut dirty_pages = self.dirty_pages.write().map_err(|e| e.to_string())?;
            dirty_pages.insert(key.clone());
        }
        let pool: RwLockReadGuard<'_, HashMap<(String, String, u32, u8), Arc<RwLock<HashBucket>>>> =
            self.pool.read().map_err(
                |e: PoisonError<
                    RwLockReadGuard<
                        '_,
                        HashMap<(String, String, u32, u8), Arc<RwLock<HashBucket>>>,
                    >,
                >| e.to_string(),
            )?;
        if let Some(page) = pool.get(&key) {
            return Ok(Arc::clone(page));
        }
        drop(pool);
        let mut buffer: Vec<u8> = vec![0; INDEX_PAGE_SIZE as usize];
        let pg_type = match is_overflow {
            0 => PageType::IndexPage,
            1 => PageType::OverflowPage,
            _ => unreachable!(),
        };
        IOEngine::read_page(db_name, table_column, &mut buffer, pg_type, page_number)?;
        let page: HashBucket = HashBucketManager::deserialize(&buffer, 0);
        let page: Arc<RwLock<HashBucket>> = Arc::new(RwLock::new(page));
        // Lock the pool for writing to insert the new page
        let mut pool: RwLockWriteGuard<
            '_,
            HashMap<(String, String, u32, u8), Arc<RwLock<HashBucket>>>,
        > = self.pool.write().map_err(
            |e: PoisonError<
                RwLockWriteGuard<'_, HashMap<(String, String, u32, u8), Arc<RwLock<HashBucket>>>>,
            >| e.to_string(),
        )?;
        // Double-check if the page was added already while we were waiting for the write lock
        if let Some(page) = pool.get(&key) {
            return Ok(Arc::clone(page));
        }
        if pool.len() > (IDX_BUF_CAP / 100 * 70) as usize {
            self.evict_pages(&mut pool)?;
        }
        pool.insert(key.clone(), Arc::clone(&page));
        let mut lru_list = self.lru_list.write().map_err(
            |e: PoisonError<RwLockWriteGuard<'_, VecDeque<(String, String, u32, u8)>>>| {
                e.to_string()
            },
        )?;
        lru_list.push_back(key.clone());
        Ok(page)
    }

    /// Moves the accessed page to the back of LRU list (most recently used).
    ///
    /// Call this after *reading/writing* a page to keep it from getting evicted soon.
    pub fn update_page_pos_lru(
        &self,
        db_name: &str,
        table_column: &str,
        is_overflow: u8,
        page_number: u32,
        mark_dirty: bool,
    ) -> Result<(), String> {
        let key = (
            db_name.to_string(),
            table_column.to_string(),
            page_number,
            is_overflow,
        );

        if mark_dirty {
            let mut dirty_pages = self.dirty_pages.write().map_err(|e| e.to_string())?;
            dirty_pages.insert(key.clone());
        }

        let mut lru_list = self.lru_list.write().map_err(|e| e.to_string())?;
        if let Some(pos) = lru_list.iter().position(|x| x == &key) {
            lru_list.remove(pos);
        }
        lru_list.push_back(key);
        Ok(())
    }

    /// Evicts least recently used pages from the pool.
    ///
    /// Only removes up to 20% of LRU list. Flushes dirty pages before removing.
    pub fn evict_pages(
        &self,
        pool: &mut RwLockWriteGuard<
            '_,
            HashMap<(String, String, u32, u8), Arc<RwLock<HashBucket>>>,
        >,
    ) -> Result<(), String> {
        let mut lru_list = self.lru_list.write().map_err(|e| e.to_string())?;
        let mut dirty_pages = self.dirty_pages.write().map_err(|e| e.to_string())?;

        if lru_list.len() < (IDX_BUF_CAP / 100 * 70) as usize {
            return Ok(());
        }

        let len = lru_list.len() / 5;
        for _ in 0..len {
            if let Some(key) = lru_list.pop_front() {
                if dirty_pages.contains(&key) {
                    self.flush_page(&key.0, &key.1, key.3, key.2, pool)?;
                    dirty_pages.remove(&key);
                }
                pool.remove(&key);
            }
        }

        Ok(())
    }

    /// Writes a single page to disk from buffer.
    ///
    /// Use this to flush a dirty page before eviction.
    pub fn flush_page(
        &self,
        db_name: &str,
        table_column: &str,
        is_overflow: u8,
        page_number: u32,
        pool: &RwLockWriteGuard<'_, HashMap<(String, String, u32, u8), Arc<RwLock<HashBucket>>>>,
    ) -> Result<(), String> {
        let key = (
            db_name.to_string(),
            table_column.to_string(),
            page_number,
            is_overflow,
        );

        if let Some(val) = pool.get(&key) {
            let page = val.read().map_err(|e| e.to_string())?;
            let mut buffer = vec![0; INDEX_PAGE_SIZE as usize];
            HashBucketManager::serialize(&page, &mut buffer, 0);

            let page_type = match is_overflow {
                0 => PageType::IndexPage,
                1 => PageType::OverflowPage,
                _ => unreachable!(),
            };
            IOEngine::update_page(db_name, table_column, &buffer, page_type, page_number)?;
        }
        Ok(())
    }
}
