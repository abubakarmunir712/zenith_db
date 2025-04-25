use crate::configs::config::Config::{PAGE_BUF_CAP, PAGE_SIZE};
use crate::storage::io::file_io::IOEngine;
use crate::{enums::page_types::PageType, storage::page::page::Page};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// PageBuffer handles in-memory caching of pages with LRU eviction and dirty page tracking.
pub struct PageBuffer {
    // Main pool of pages in memory. Key: (db_name, table_name, page_number)
    pool: Arc<RwLock<HashMap<(String, String, u32), Arc<RwLock<Page>>>>>,
    // Tracks order of page usage for eviction (LRU).
    lru_list: Arc<RwLock<VecDeque<(String, String, u32)>>>,
    // Keeps track of dirty (modified) pages.
    dirty_pages: Arc<RwLock<HashSet<(String, String, u32)>>>,
}

impl PageBuffer {
    /// Creates a new page buffer with empty cache.
    pub fn new() -> Arc<PageBuffer> {
        Arc::new(PageBuffer {
            pool: Arc::new(RwLock::new(HashMap::with_capacity(PAGE_BUF_CAP as usize))),
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
        table_name: &str,
        page_number: u32,
        mark_dirty: bool,
    ) -> Result<Arc<RwLock<Page>>, String> {
        let key = (db_name.to_string(), table_name.to_string(), page_number);
        if mark_dirty {
            let mut dirty_pages = self.dirty_pages.write().map_err(|e| e.to_string())?;
            dirty_pages.insert(key.clone());
        }
        let pool: RwLockReadGuard<'_, HashMap<(String, String, u32), Arc<RwLock<Page>>>> =
            self.pool.read().map_err(
                |e: PoisonError<
                    RwLockReadGuard<'_, HashMap<(String, String, u32), Arc<RwLock<Page>>>>,
                >| e.to_string(),
            )?;
        if let Some(page) = pool.get(&key) {
            return Ok(Arc::clone(page));
        }
        drop(pool);
        let mut buffer = vec![0; PAGE_SIZE as usize];
        IOEngine::read_page(
            db_name,
            table_name,
            &mut buffer,
            PageType::DataPage,
            page_number,
        )?;
        let page = Page::deserialize(&buffer);
        let page: Arc<RwLock<Page>> = Arc::new(RwLock::new(page));
        // Lock the pool for writing to insert the new page
        let mut pool: RwLockWriteGuard<'_, HashMap<(String, String, u32), Arc<RwLock<Page>>>> =
            self.pool.write().map_err(
                |e: PoisonError<
                    RwLockWriteGuard<'_, HashMap<(String, String, u32), Arc<RwLock<Page>>>>,
                >| e.to_string(),
            )?;
        // Double-check if the page was added already while we were waiting for the write lock
        if let Some(page) = pool.get(&key) {
            return Ok(Arc::clone(page));
        }
        if pool.len() > (PAGE_BUF_CAP / 100 * 70) as usize {
            self.evict_pages(&mut pool)?;
        }
        pool.insert(key, Arc::clone(&page));
        let mut lru_list = self.lru_list.write().map_err(
            |e: PoisonError<RwLockWriteGuard<'_, VecDeque<(String, String, u32)>>>| e.to_string(),
        )?;
        lru_list.push_back((db_name.to_string(), table_name.to_string(), page_number));
        Ok(page)
    }

    /// Moves the accessed page to the back of LRU list (most recently used).
    ///
    /// Call this after *reading/writing* a page to keep it from getting evicted soon.
    pub fn update_page_pos_lru(
        &self,
        db_name: &str,
        table_name: &str,
        page_number: u32,
    ) -> Result<(), String> {
        let mut lru_list = self.lru_list.write().map_err(
            |e: PoisonError<RwLockWriteGuard<'_, VecDeque<(String, String, u32)>>>| e.to_string(),
        )?;

        if let Some(pos) = lru_list
            .iter()
            .position(|(db, file, id)| db == db_name && file == table_name && *id == page_number)
        {
            lru_list.remove(pos);
        }
        lru_list.push_back((db_name.to_string(), table_name.to_string(), page_number));

        Ok(())
    }

    /// Evicts least recently used pages from the pool.
    ///
    /// Only removes up to 20% of LRU list. Flushes dirty pages before removing.
    pub fn evict_pages(
        &self,
        pool: &mut RwLockWriteGuard<'_, HashMap<(String, String, u32), Arc<RwLock<Page>>>>,
    ) -> Result<(), String> {
        let mut lru_list: RwLockWriteGuard<'_, VecDeque<(String, String, u32)>> =
            self.lru_list.write().map_err(
                |e: PoisonError<RwLockWriteGuard<'_, VecDeque<(String, String, u32)>>>| {
                    e.to_string()
                },
            )?;
        let mut dirty_pages = self.dirty_pages.write().map_err(
            |e: PoisonError<RwLockWriteGuard<'_, HashSet<(String, String, u32)>>>| e.to_string(),
        )?;

        if lru_list.len() < (PAGE_BUF_CAP / 100 * 70) as usize {
            return Ok(());
        }

        let len = lru_list.len() / 5; // Evict 20 percent pages from lru
        for i in 0..len {
            if let Some(key) = lru_list.pop_front() {
                if dirty_pages.contains(&key) {
                    self.flush_page(&key.0, &key.1, key.2, &pool);
                    pool.remove(&key);
                    dirty_pages.remove(&key);
                }
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
        table_name: &str,
        page_number: u32,
        pool: &RwLockWriteGuard<'_, HashMap<(String, String, u32), Arc<RwLock<Page>>>>,
    ) -> Result<(), String> {
        let key = (db_name.to_string(), table_name.to_string(), page_number);
        if let Some(val) = pool.get(&key) {
            let page: RwLockReadGuard<'_, Page> = val
                .read()
                .map_err(|e: PoisonError<RwLockReadGuard<'_, Page>>| e.to_string())?;

            let mut buffer = vec![0; PAGE_SIZE as usize];
            page.serialize(&mut buffer);
            IOEngine::update_page(
                db_name,
                table_name,
                &buffer,
                PageType::DataPage,
                page_number,
            )?;
        }
        Ok(())
    }
}
