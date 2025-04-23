use ZenithDB::configs::config::Config::PAGE_SIZE;
use ZenithDB::storage::page::page::Page;
use ZenithDB::storage::page::page_header::PageHeader;
use ZenithDB::storage::page::page_manager::PageManager;
use ZenithDB::storage::page::slot::Slot;

#[test]
#[rustfmt::skip]
/// Tests if `PageHeader` correctly serializes and deserializes, preserving all field values.
fn test_page_header_serialize_deserialize() {
    // Create an instance of PageHeader with arbitrary values
    let original = PageHeader::new(42, 1337, 100, 5, 2048);

    let mut buffer = [0u8; PAGE_SIZE as usize]; // Create a buffer of the appropriate size
    original.serialize(&mut buffer); // Serialize the original PageHeader into the buffer

    let deserialized = PageHeader::deserialize(&buffer); // Deserialize the buffer back into a PageHeader

    // Assert that all fields match after serialization and deserialization
    assert_eq!(original.page_id(), deserialized.page_id());
    assert_eq!(original.lsn(), deserialized.lsn());
    assert_eq!(original.free_space_offset(), deserialized.free_space_offset());
    assert_eq!(original.num_of_tuples(), deserialized.num_of_tuples());
    assert_eq!(original.slot_table_offset(), deserialized.slot_table_offset());
}

#[test]
#[rustfmt::skip]
/// Verifies that a `Slot` struct can be serialized into bytes and deserialized without data loss.
fn test_slot_serialize_deserialize() {
    // Create an instance of Slot with arbitrary values
    let original = Slot::new(
        1234, // record_offset
        56,   // record_size
        0,    // is_deleted (0 means not deleted)
        78,   // total_size
    );

    let mut buffer = [0u8; 8]; // Create a buffer to hold the serialized data
    original.serialize(&mut buffer); // Serialize the original Slot into the buffer

    let deserialized = Slot::deserialize(&buffer); // Deserialize the buffer back into a Slot

    // Assert that all fields match after serialization and deserialization
    assert_eq!(original.record_offset(), deserialized.record_offset());
    assert_eq!(original.record_size(), deserialized.record_size());
    assert_eq!(original.is_deleted(), deserialized.is_deleted());
    assert_eq!(original.total_size(), deserialized.total_size());
}

#[test]
/// Checks full-page serialization: inserting fake data and a slot, then deserializing and verifying consistency.
fn test_page_serialize_deserialize() {
    let page_id = 42;
    let lsn = 1000;

    // Init Page normally
    let mut page = Page::new(page_id, lsn);
    assert_eq!(page.data().len(), 4076);
    assert_eq!(page.data().capacity(), 4076);

    // Create a fake record and push it into data
    let fake_record: Vec<u8> = vec![1, 2, 3, 4, 5];
    page.data_as_mut()[0..5].copy_from_slice(&fake_record);

    // Update header + slot info manually like it would happen in real insert``
    page.page_header_as_mut().inc_num_of_tuples(1);
    page.page_header_as_mut().set_free_space_offset(25);
    page.page_header_as_mut()
        .set_slot_table_offset(PAGE_SIZE as u16 - 8);
    page.data_as_mut().resize(PAGE_SIZE as usize - 20 - 8, 0);
    assert_eq!(page.data().len(), 4068);
    assert_eq!(page.data().capacity(), 4076);

    // Add a dummy slot
    let slot = Slot::new(0, 5, 0, 5);
    page.slot_table_as_mut().push(slot);

    // Serialize it
    let mut buffer: [u8; PAGE_SIZE as usize] = [0; PAGE_SIZE as usize];
    page.serialize(&mut buffer);

    // Deserialize it back
    let deserialized_page = Page::deserialize(&buffer);

    // Assertions
    assert_eq!(page.data(), deserialized_page.data());
    assert_eq!(deserialized_page.page_header().page_id(), page_id);
    assert_eq!(deserialized_page.page_header().lsn(), lsn);
    assert_eq!(deserialized_page.page_header().num_of_tuples(), 1);
    assert_eq!(deserialized_page.slot_table().len(), 1);
    assert_eq!(deserialized_page.slot_table()[0].record_offset(), 0);
    assert_eq!(deserialized_page.slot_table()[0].record_size(), 5);
    assert_eq!(deserialized_page.data()[0..5], fake_record[..]);
}

#[test]
#[rustfmt::skip]
/// Validates `PageManager` slot operations: insert, mark as deleted, reuse deleted slot correctly.
/// Validates `PageManager` free space calculation
fn test_page_manager() {
    let mut page = Page::new(0, 0);
    // Insert slot
    PageManager::insert_slot(&mut page, 0, 5);
    assert_eq!(page.data().len(), 4068);
    assert_eq!(page.slot_table().len(), 1);
    assert_eq!(page.page_header().free_space_offset(), 25);
    // Insert slot
    PageManager::insert_slot(&mut page, 5, 5);
    assert_eq!(page.data().len(), 4068 - 8);
    assert_eq!(page.slot_table().len(), 2);
    assert_eq!(page.page_header().free_space_offset(), 30);
    // Delete slot
    PageManager::mark_slot_as_deleted(&mut page, 5);

    assert_eq!(PageManager::get_deleted_slots_offsets(&mut page), vec![(5, 5)]);
    assert_eq!(PageManager::free_space_table(&mut page),vec![(5, 5), (10, 4050)]);
    assert_eq!(page.slot_table().len(), 2);
    assert_eq!(page.slot_table()[1].is_deleted(), 1);

    // Insert slot in place of previously deleted slot
    PageManager::insert_slot(&mut page, 5, 3);

    assert_eq!(PageManager::get_deleted_slots_offsets(&mut page), Vec::new());
    assert_eq!(PageManager::free_space_table(&mut page),vec![(10, 4050)]);
    assert_eq!(page.slot_table().len(), 2);
    assert_eq!(page.slot_table()[1].is_deleted(), 0);
    assert_eq!(page.slot_table()[1].total_size(), 5);
    assert_eq!(page.data().len(), 4068 - 8);
    assert_eq!(page.slot_table().len(), 2);
    assert_eq!(page.page_header().free_space_offset(), 30);
}
