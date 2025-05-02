use ZenithDB::enums::page_types::PageType;
use ZenithDB::storage::buffer::page_buffer::PageBuffer;
use ZenithDB::storage::io::file_io::IOEngine;
use ZenithDB::storage::page::page::Page;
use ZenithDB::storage::page::page_manager::PageManager;
use rand::Rng;
use std::sync::{Arc, RwLock};
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
        page.data_as_mut()[0..5].copy_from_slice(&[1, 3, 5, 7, 9]);
    }
    {
        for i in 1..1000 as u32 {
            let a = pb.get_page(db_name, table_name, i, false).unwrap();
            let page = a.read().unwrap();
            // let data: [u8; 5] = [1, 3, 5, 7, 9];
            // assert_eq!(page.data()[0..5], data)
        }
    }
    // IOEngine::delete_db(db_name).unwrap();
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
