use std::collections::{HashMap, VecDeque};
use std::io::Error;

use crate::storage::catalog::catalog::CatalogTable;
use super::super::io::file_io::IOEngine;

pub struct CatalogBufferManager {
    pool: HashMap<(String, String), CatalogTable>, // (database_name, table_name)
    lru_list: VecDeque<(String, String)>,
    capacity: usize,
}

impl CatalogBufferManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: HashMap::new(),
            lru_list: VecDeque::new(),
            capacity,
        }
    }

    pub fn fetch_catalog(
        &mut self,
        database_name: &str,
        table_name: &str,
    ) -> Result<&mut CatalogTable, Error> {
        let key = (database_name.to_string(), table_name.to_string());

        if self.pool.contains_key(&key) {
            self.touch(database_name, table_name);
            return Ok(self.pool.get_mut(&key).unwrap());
        }

        if self.pool.len() >= self.capacity {
            self.evict_catalog()?;
        }

        let table = IOEngine::load_catalog_table(database_name, table_name)?;
        self.pool.insert(key.clone(), table);
        self.lru_list.push_back(key.clone());

        Ok(self.pool.get_mut(&key).unwrap())
    }

    pub fn insert_catalog(
        &mut self,
        database_name: &str,
        table_name: &str,
        mut catalog: CatalogTable,
    ) -> Result<(), Error> {
        let key = (database_name.to_string(), table_name.to_string());

        if self.pool.len() >= self.capacity && !self.pool.contains_key(&key) {
            self.evict_catalog()?;
        }

        catalog.set_dirty(true); // Mark it dirty on insert
        self.pool.insert(key.clone(), catalog);
        self.touch(database_name, table_name);
        Ok(())
    }

    fn touch(&mut self, database_name: &str, table_name: &str) {
        let key = (database_name.to_string(), table_name.to_string());
        if let Some(pos) = self.lru_list.iter().position(|k| k == &key) {
            self.lru_list.remove(pos);
        }
        self.lru_list.push_back(key);
    }

    fn evict_catalog(&mut self) -> Result<(), Error> {
        if let Some(oldest_key) = self.lru_list.pop_front() {
            if let Some(catalog) = self.pool.remove(&oldest_key) {
                if catalog.is_dirty() {
                    IOEngine::save_catalog_table(&catalog)?;
                }
            }
        }
        Ok(())
    }

    pub fn flush_all(&mut self) -> Result<(), Error> {
        for (_, catalog) in self.pool.iter_mut() {
            if catalog.is_dirty() {
                IOEngine::save_catalog_table(catalog)?;
                catalog.set_dirty(false);
            }
        }
        Ok(())
    }
}
