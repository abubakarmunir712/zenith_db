use crate::configs::config::Config::{CATLOG_BUF_CAP, CATLOG_PAGE_SIZE};
use crate::enums::types::catalog_types::CatalogData;
use crate::enums::types::catalog_types::CatalogType;
use crate::enums::types::page_types::PageType;
use crate::storage::io::file_io::IOEngine;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

/// PageBuffer handles in-memory caching of pages with LRU eviction and dirty page tracking.
pub struct CatalogBuffer {
    // Main pool of catalog pages in memory. Key: (db_name, page_type, page_number)
    pool: Arc<RwLock<HashMap<(String, CatalogType, u32), Arc<RwLock<CatalogData>>>>>,
    // Tracks order of page usage for eviction (LRU).
    lru_list: Arc<RwLock<VecDeque<(String, CatalogType, u32)>>>,
    // Keeps track of dirty (modified) pages.
    dirty_pages: Arc<RwLock<HashSet<(String, CatalogType, u32)>>>,
}

impl CatalogBuffer {
    /// Creates a new catalog buffer with empty cache.
    pub fn new() -> Arc<CatalogBuffer> {
        Arc::new(CatalogBuffer {
            pool: Arc::new(RwLock::new(HashMap::with_capacity(CATLOG_BUF_CAP as usize))),
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
        page_type: CatalogType,
        page_number: u32,
        mark_dirty: bool,
    ) -> Result<Arc<RwLock<CatalogData>>, String> {
        let key = (db_name.to_string(), page_type, page_number);
        if mark_dirty {
            let mut dirty_pages = self.dirty_pages.write().map_err(|e| e.to_string())?;
            dirty_pages.insert(key.clone());
        }
        let pool: RwLockReadGuard<
            '_,
            HashMap<(String, CatalogType, u32), Arc<RwLock<CatalogData>>>,
        > = self.pool.read().map_err(
            |e: PoisonError<
                RwLockReadGuard<'_, HashMap<(String, CatalogType, u32), Arc<RwLock<CatalogData>>>>,
            >| e.to_string(),
        )?;
        if let Some(page) = pool.get(&key) {
            return Ok(Arc::clone(page));
        }
        drop(pool);
        let mut buffer: Vec<u8> = vec![0; CATLOG_PAGE_SIZE as usize];
        let pg_type = match page_type {
            CatalogType::RefMap => PageType::RefPage,
            _ => PageType::CatlogPage,
        };
        IOEngine::read_page(db_name, db_name, &mut buffer, pg_type, page_number)?;
        let page: CatalogData = CatalogData::deserialize(&buffer, page_type);
        let page: Arc<RwLock<CatalogData>> = Arc::new(RwLock::new(page));
        // Lock the pool for writing to insert the new page
        let mut pool: RwLockWriteGuard<
            '_,
            HashMap<(String, CatalogType, u32), Arc<RwLock<CatalogData>>>,
        > = self.pool.write().map_err(
            |e: PoisonError<
                RwLockWriteGuard<'_, HashMap<(String, CatalogType, u32), Arc<RwLock<CatalogData>>>>,
            >| e.to_string(),
        )?;
        // Double-check if the page was added already while we were waiting for the write lock
        if let Some(page) = pool.get(&key) {
            return Ok(Arc::clone(page));
        }
        if pool.len() > (CATLOG_BUF_CAP / 100 * 70) as usize {
            self.evict_pages(&mut pool)?;
        }
        pool.insert(key, Arc::clone(&page));
        let mut lru_list = self.lru_list.write().map_err(
            |e: PoisonError<RwLockWriteGuard<'_, VecDeque<(String, CatalogType, u32)>>>| {
                e.to_string()
            },
        )?;
        lru_list.push_back((db_name.to_string(), page_type, page_number));
        Ok(page)
    }

    /// Moves the accessed page to the back of LRU list (most recently used).
    ///
    /// Call this after *reading/writing* a page to keep it from getting evicted soon.
    pub fn update_page_pos_lru(
        &self,
        db_name: &str,
        page_type: CatalogType,
        page_number: u32,
    ) -> Result<(), String> {
        let mut lru_list = self.lru_list.write().map_err(
            |e: PoisonError<RwLockWriteGuard<'_, VecDeque<(String, CatalogType, u32)>>>| {
                e.to_string()
            },
        )?;

        if let Some(pos) = lru_list.iter().position(|(db, pg_type, id)| {
            db == db_name && *pg_type == page_type && *id == page_number
        }) {
            lru_list.remove(pos);
        }
        lru_list.push_back((db_name.to_string(), page_type, page_number));

        Ok(())
    }

    /// Evicts least recently used pages from the pool.
    ///
    /// Only removes up to 20% of LRU list. Flushes dirty pages before removing.
    pub fn evict_pages(
        &self,
        pool: &mut RwLockWriteGuard<
            '_,
            HashMap<(String, CatalogType, u32), Arc<RwLock<CatalogData>>>,
        >,
    ) -> Result<(), String> {
        let mut lru_list: RwLockWriteGuard<'_, VecDeque<(String, CatalogType, u32)>> =
            self.lru_list.write().map_err(
                |e: PoisonError<RwLockWriteGuard<'_, VecDeque<(String, CatalogType, u32)>>>| {
                    e.to_string()
                },
            )?;
        let mut dirty_pages = self.dirty_pages.write().map_err(
            |e: PoisonError<RwLockWriteGuard<'_, HashSet<(String, CatalogType, u32)>>>| {
                e.to_string()
            },
        )?;

        if lru_list.len() < (CATLOG_BUF_CAP / 100 * 70) as usize {
            return Ok(());
        }

        let len = lru_list.len() / 5; // Evict 20 percent pages from lru
        for i in 0..len {
            if let Some(key) = lru_list.pop_front() {
                if dirty_pages.contains(&key) {
                    self.flush_page(&key.0, key.1, key.2, &pool)?;
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
        page_type: CatalogType,
        page_number: u32,
        pool: &RwLockWriteGuard<'_, HashMap<(String, CatalogType, u32), Arc<RwLock<CatalogData>>>>,
    ) -> Result<(), String> {
        let key = (db_name.to_string(), page_type, page_number);
        if let Some(val) = pool.get(&key) {
            let page: RwLockReadGuard<'_, CatalogData> = val
                .read()
                .map_err(|e: PoisonError<RwLockReadGuard<'_, CatalogData>>| e.to_string())?;

            let mut buffer = vec![0; CATLOG_PAGE_SIZE as usize];
            page.serialize(&mut buffer);
            IOEngine::update_page(db_name, db_name, &buffer, PageType::CatlogPage, page_number)?;
        }
        Ok(())
    }
}
