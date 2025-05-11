use ZenithDB::configs::config::Config::CATLOG_PAGE_SIZE;
use ZenithDB::enums::types::cascading_type::ForeignKeyAction;
use ZenithDB::enums::types::catalog_types::{CatalogData, CatalogType};
use ZenithDB::enums::types::datatypes::DataType;
use ZenithDB::enums::types::page_types::PageType;
use ZenithDB::storage::buffer::catalog_buffer::CatalogBuffer;
use ZenithDB::storage::buffer::page_buffer::PageBuffer;
use ZenithDB::storage::catalog::entries::column_entry::ColumnEntry;
use ZenithDB::storage::catalog::entries::ref_entry::{RefEntry, RefPair};
use ZenithDB::storage::catalog::entries::table_entry::TableEntry;
use ZenithDB::storage::catalog::maps::column_map::ColumnMap;
use ZenithDB::storage::catalog::maps::ref_map::RefMap;
use ZenithDB::storage::catalog::maps::table_map::TableMap;
use ZenithDB::storage::io::file_io::IOEngine;
use ZenithDB::storage::page::page::Page;
use ZenithDB::storage::page::page_manager::PageManager;
use rand::Rng;
use std::sync::{Arc, RwLockReadGuard};
use std::thread;
use std::time::{Duration, Instant};

/// Updates a page's data *in-place* in buffer, writes it back, then reads again to verify it's actually updated.
#[test]
fn test_page_update_in_place() {
    let db_name = "test_db1";
    let table_name = "test_table";

    // Setup the database and table
    IOEngine::create_db(db_name).unwrap();
    IOEngine::create_table(db_name, table_name).unwrap();
    let pb = PageBuffer::new();
    {
        for i in 0..1000 {
            let mut page = Page::new(i as u32, i);
            page.data_as_mut()[0..5].copy_from_slice(&[0, 1, 2, 3, 4]);
            PageManager::insert_slot(&mut page, 0, 5);
            let mut buffer: [u8; 4096] = [0; 4096];
            page.serialize(&mut buffer);

            IOEngine::append_page(db_name, table_name, &buffer, PageType::DataPage).unwrap();
        }
        let a = pb.get_page(db_name, table_name, 0, true).unwrap();
        let mut page = a.write().unwrap();
        page.data_as_mut()[0..5].copy_from_slice(&[1, 1, 2, 3, 4]);
    }
    {
        for i in 1..1000 as u32 {
            let a = pb.get_page(db_name, table_name, i, false).unwrap();
            let page = a.read().unwrap();
            let data: [u8; 5] = [0, 1, 2, 3, 4];
            assert_eq!(page.data()[0..5], data)
        }
        let a = pb.get_page(db_name, table_name, 0, false).unwrap();
        let page = a.read().unwrap();
        let data: [u8; 5] = [1, 1, 2, 3, 4];
        assert_eq!(page.data()[0..5], data)
    }
    IOEngine::delete_db(db_name).unwrap();
}

// Loads 5000 pages concurrently into the buffer, tests eviction behavior.
// Each thread loads a page, dirties it, then prints some data length for vibes.
// Great for stress testing buffer manager.
#[test]
fn test_page_buffer_and_eviction() {
    let db_name = "test_db2";
    let table_name = "test_table";

    // Setup the database and table
    IOEngine::create_db(db_name).unwrap();
    IOEngine::create_table(db_name, table_name).unwrap();

    // Insert 5000 pages with varying data sizes
    for i in 0..5000 {
        let data_size = match i % 5 {
            // Vary the data size every 5 pages
            0 => 10,
            1 => 7,
            2 => 15,
            3 => 20,
            _ => 8,
        };

        let mut page = Page::new(i, i as u64);

        // Create fake data of the size defined above
        let fake_data: Vec<u8> = (0..data_size).map(|x| (x * 2) as u8).collect();

        // Insert data into the page
        page.data_as_mut()[0..data_size].copy_from_slice(&fake_data);

        // Insert the slot with the correct offset (start at 0) and the record size
        PageManager::insert_slot(&mut page, 0, data_size as u16);

        // Serialize and append the page
        let mut buffer = [0u8; 4096];
        page.serialize(&mut buffer);
        IOEngine::append_page(db_name, table_name, &buffer, PageType::DataPage).unwrap();
    }

    // Create a new PageBuffer instance wrapped in Arc and Mutex for safe concurrent access
    let page_buffer = PageBuffer::new();

    // Spawn threads to load pages concurrently
    let mut handles = vec![];

    for i in 0..5000 {
        let db_name = db_name.to_string();
        let table_name = table_name.to_string();
        let page_buffer = Arc::clone(&page_buffer); // Clone the Arc for sharing

        // Create a thread to load the page
        let handle = thread::spawn(move || {
            // Load page for the given page number
            let result = page_buffer.get_page(&db_name, &table_name, i, true);
            match result {
                Ok(page) => {
                    let mut page = page.write().unwrap();
                    page.set_is_dirty(true);

                    // Print the data to verify
                    let sleep_duration = rand::thread_rng().gen_range(1..=50);
                    thread::sleep(Duration::from_millis(sleep_duration));

                    // Print the first 5 bytes of the page as an example (can modify for more data)
                    println!(
                        "Page {}: Length: {:?}, Sleep Duration: {:?}",
                        i,
                        &page.page_header().free_space_offset() - 20,
                        sleep_duration
                    );
                }
                Err(e) => {
                    eprintln!("Failed to load page {}: {}", i, e);
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    IOEngine::delete_db(db_name).unwrap();
}

fn load_pages_and_measure_time(
    page_buffer: &Arc<PageBuffer>,
    db_name: &str,
    table_name: &str,
    num_pages: usize,
) -> (Duration, Duration) {
    let read_start = Instant::now();

    // First, perform a read operation for all pages (cold cache)
    for i in 0..num_pages {
        let result = page_buffer.get_page(db_name, table_name, i as u32, true);
        match result {
            Ok(page) => {
                let _page = page.read().unwrap();
            }
            Err(e) => {
                eprintln!("Failed to load page {}: {}", i, e);
            }
        }
    }

    let read_duration = read_start.elapsed();

    let write_start = Instant::now();

    // Then, again load page second time to see performance boost due to cache hit
    for i in 0..num_pages {
        let page_buffer = page_buffer;
        let result = page_buffer.get_page(db_name, table_name, i as u32, true);
        match result {
            Ok(page) => {
                let page = page.read().unwrap();
            }
            Err(e) => {
                eprintln!("Failed to load page {}: {}", i, e);
            }
        }
    }

    let write_duration = write_start.elapsed();

    (read_duration, write_duration)
}

// Benchmarks page loading time with/without OS cache using `load_pages_and_measure_time()`.
// First inserts 500 pages with variable data sizes, then times two full scans.
// Prints timing diff (cache miss vs hit).
#[test]
fn test_page_buffer_time() {
    let db_name = "test_db3";
    let table_name = "test_table";

    // Setup the database and table
    IOEngine::create_db(db_name).unwrap();
    IOEngine::create_table(db_name, table_name).unwrap();

    // Insert 500 pages with varying data sizes
    for i in 0..500 {
        let data_size = match i % 5 {
            // Vary the data size every 5 pages
            0 => 10,
            1 => 7,
            2 => 15,
            3 => 20,
            _ => 8,
        };

        let mut page = Page::new(i, i as u64);

        // Create fake data of the size defined above
        let fake_data: Vec<u8> = (0..data_size).map(|x| (x * 2) as u8).collect();

        // Insert data into the page
        page.data_as_mut()[0..data_size].copy_from_slice(&fake_data);

        // Insert the slot with the correct offset (start at 0) and the record size
        PageManager::insert_slot(&mut page, 0, data_size as u16);

        // Serialize and append the page
        let mut buffer = [0u8; 4096];
        page.serialize(&mut buffer);
        IOEngine::append_page(db_name, table_name, &buffer, PageType::DataPage).unwrap();
    }

    // Create a new PageBuffer instance wrapped in Arc and RwLock for safe concurrent access
    let page_buffer = PageBuffer::new();

    // Load pages and measure time for cache hit and cache miss
    let (read_duration, write_duration) =
        load_pages_and_measure_time(&page_buffer, db_name, table_name, 500);

    // Print out the timings to compare performance
    println!("Time for cache miss: {:?}", read_duration);
    println!("Time for cache hit: {:?}", write_duration);

    // Clean up
    IOEngine::delete_db(db_name).unwrap();
}

#[test]
fn test_catalog_buffer() {
    let db_name = "test_db_4";
    let table_name = "Abc".to_string();
    IOEngine::create_db(db_name).unwrap();
    let catlog_buffer = CatalogBuffer::new();

    // Table map
    let mut table_map = TableMap::new();
    let mut table_entry = TableEntry::new(table_name.clone(), 1).unwrap();
    table_entry.increase_columns();
    table_entry.increase_no_of_cols_in_pk();
    table_entry.increase_no_of_cols_in_pk();
    table_map.create_table(table_entry);
    let mut buffer: [u8; 32768] = [0; CATLOG_PAGE_SIZE as usize];
    table_map.serialize(&mut buffer);

    IOEngine::append_page(db_name, db_name, &buffer, PageType::CatlogPage).unwrap();

    let page = catlog_buffer
        .get_page(db_name, CatalogType::TableMap, 0, false)
        .unwrap();
    let page: RwLockReadGuard<'_, CatalogData> = page.read().unwrap();
    if let CatalogData::TableMap(map) = &*page {
        assert_eq!(map.no_of_tables(), 1);
        assert_eq!(map.get_table(&table_name).unwrap().columns(), 1);
        assert_eq!(
            map.get_table(&table_name)
                .unwrap()
                .no_of_cols_in_primary_key(),
            2
        );
    } else {
        panic!("Table name")
    }

    // Column Map
    let col_name = "col_1";
    let mut column_map = ColumnMap::new();
    let mut column_entry = ColumnEntry::new(col_name.to_string(), 1, DataType::CHAR, 20).unwrap();
    column_entry.make_foreign();
    column_entry.make_unique();
    column_map.create_column(column_entry);
    let mut buffer: [u8; 32768] = [0; CATLOG_PAGE_SIZE as usize];
    column_map.serialize(&mut buffer);
    IOEngine::append_page(db_name, db_name, &buffer, PageType::CatlogPage).unwrap();

    let page = catlog_buffer
        .get_page(db_name, CatalogType::ColumnMap, 1, false)
        .unwrap();
    let page: RwLockReadGuard<'_, CatalogData> = page.read().unwrap();
    if let CatalogData::ColumnMap(map) = &*page {
        assert_eq!(map.no_of_columns(), 1);
        assert_eq!(map.map().get(col_name).unwrap().is_foreign_key(), true);
        assert_eq!(map.map().get(col_name).unwrap().column_name(), col_name);
    } else {
        panic!("Table name")
    }

    // Ref Map
    let mut ref_map = RefMap::new();
    let mut ref_pair = RefPair::new(Some(1), Some(2), Some(3), Some(4));
    let ref_entry = RefEntry::new(vec![ref_pair], ForeignKeyAction::Restrict).unwrap();
    assert_eq!(ref_entry.references().len(), 1);
    ref_map.insert_ref(ref_entry).unwrap();
    let mut buffer: [u8; 32768] = [0; CATLOG_PAGE_SIZE as usize];
    ref_map.serialize(&mut buffer);
    IOEngine::append_page(db_name, db_name, &buffer, PageType::RefPage).unwrap();

    let page = catlog_buffer
        .get_page(db_name, CatalogType::RefMap, 0, false)
        .unwrap();
    let page: RwLockReadGuard<'_, CatalogData> = page.read().unwrap();
    if let CatalogData::RefMap(map) = &*page {
        assert_eq!(map.no_of_ref(), 1);
        assert_eq!(
            map.map()[0].cascading_type().to_oid(),
            ForeignKeyAction::Restrict.to_oid()
        );
        assert_eq!(map.map()[0].references().len(), 1);
    } else {
        panic!("Table name")
    }

    IOEngine::delete_db(db_name).unwrap();
}

#[test]
fn test_catalog_page_update_in_place() {
    let db_name = "test_db_cat_update";
    let table_name = "test_table".to_string();
    IOEngine::create_db(db_name).unwrap();
    let catlog_buffer = CatalogBuffer::new();

    // Write 1000 catalog pages (TableMap)
    for i in 0..1000 {
        let mut table_map = TableMap::new();
        let mut table_entry = TableEntry::new(format!("{}_{}", table_name, i), i as u16).unwrap();
        table_entry.increase_columns();
        table_map.create_table(table_entry);

        let mut buffer: [u8; CATLOG_PAGE_SIZE as usize] = [0; CATLOG_PAGE_SIZE as usize];
        table_map.serialize(&mut buffer);

        IOEngine::append_page(db_name, db_name, &buffer, PageType::CatlogPage).unwrap();
    }

    // Update page 0
    {
        let page = catlog_buffer
            .get_page(db_name, CatalogType::TableMap, 0, true)
            .unwrap();
        let mut page = page.write().unwrap();

        if let CatalogData::TableMap(map) = &mut *page {
            let table = map.map_mut().get_mut(&format!("{}_0", table_name)).unwrap();
            table.increase_columns(); // +1 column
        } else {
            panic!("Expected TableMap");
        }
    }

    // Check pages 1..999 remain same
    {
        for i in 1..1000 {
            let page = catlog_buffer
                .get_page(db_name, CatalogType::TableMap, i, false)
                .unwrap();
            let page = page.read().unwrap();

            if let CatalogData::TableMap(map) = &*page {
                let table = map.get_table(&format!("{}_{}", table_name, i)).unwrap();
                assert_eq!(table.columns(), 1);
            } else {
                panic!("Expected TableMap");
            }
        }
    }

    // Recheck page 0 to confirm update
    {
        let page = catlog_buffer
            .get_page(db_name, CatalogType::TableMap, 0, false)
            .unwrap();
        let page = page.read().unwrap();

        if let CatalogData::TableMap(map) = &*page {
            let table = map.get_table(&format!("{}_0", table_name)).unwrap();
            assert_eq!(table.columns(), 2); // update must be visible
        } else {
            panic!("Expected TableMap");
        }
    }

    IOEngine::delete_db(db_name).unwrap();
}

#[test]
fn test_catalog_multithreaded_access() {
    use std::sync::Arc;
    use std::thread;

    let db_name = "test_db_threaded";
    IOEngine::create_db(db_name).unwrap();
    let catlog_buffer = Arc::new(CatalogBuffer::new());

    // Add 10 catalog pages for testing
    for i in 0..10 {
        let mut table_map = TableMap::new();
        let mut table_entry = TableEntry::new(format!("table_{}", i), i).unwrap();
        table_entry.increase_columns();
        table_map.create_table(table_entry);

        let mut buffer: [u8; CATLOG_PAGE_SIZE as usize] = [0; CATLOG_PAGE_SIZE as usize];
        table_map.serialize(&mut buffer);
        IOEngine::append_page(db_name, db_name, &buffer, PageType::CatlogPage).unwrap();
    }

    // Spawn 10 threads, each reading one page
    let mut handles = vec![];
    for i in 0..10 {
        let catlog_buffer = Arc::clone(&catlog_buffer);
        let db_name = db_name.to_string();
        handles.push(thread::spawn(move || {
            let page = catlog_buffer
                .get_page(&db_name, CatalogType::TableMap, i, false)
                .unwrap();
            let page = page.read().unwrap();
            if let CatalogData::TableMap(map) = &*page {
                let table = map.get_table(&format!("table_{}", i)).unwrap();
                println!("{}", i);
                assert_eq!(table.columns(), 1);
            } else {
                panic!("Expected TableMap");
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    IOEngine::delete_db(db_name).unwrap();
}
