use ZenithDB::oid::oid_manager::*;
#[test]
fn test_generate_oid() {
    let bitmask: Vec<u8> = vec![0b11111111, 0b11111111, 0b11111111]; // All OIDs are used (OIDs 1 to 24)
    let result = generate_oid(&bitmask);
    assert_eq!(result, None); // All OIDs are used, so None should be returned

    let bitmask: Vec<u8> = vec![0b00000000, 0b00000000, 0b00000000]; // No OIDs are used (OIDs 1 to 24 free)
    let result = generate_oid(&bitmask);
    assert_eq!(result, Some(1)); // The first available OID is 1

    let bitmask: Vec<u8> = vec![0b11111110, 0b00000000, 0b00000000]; // OID 1 to 7 is used, OID 8 is free
    let result = generate_oid(&bitmask);
    assert_eq!(result, Some(8)); // The first available OID is 8
}

#[test]
fn test_delete_oid() {
    let mut bitmask: Vec<u8> = vec![0b00000000, 0b00000000, 0b00000000]; // No OIDs are used (OIDs 1 to 24 free)
    
    delete_oid(&mut bitmask, 3); // Mark OID 3 as deleted
    assert_eq!(bitmask[0], 0b00100000); // OID 3 is marked as deleted (third bit of the first byte)

    delete_oid(&mut bitmask, 10); // Mark OID 10 as deleted
    assert_eq!(bitmask[1], 0b01000000); // OID 10 is marked as deleted (second bit of the second byte)
}

#[test]
fn test_undelete_oid() {
    let mut bitmask: Vec<u8> = vec![0b00011111, 0b10000000, 0b00000000]; // OIDs 1 to 5 are deleted, the rest are free

    undelete_oid(&mut bitmask, 4); // Undelete OID 4
    assert_eq!(bitmask[0], 0b00001111); // OID 4 is now undeleted (fourth bit of the first byte is 0)

    undelete_oid(&mut bitmask, 9); // Undelete OID 10
    assert_eq!(bitmask[1], 0b00000000); // OID 10 is now undeleted (second bit of the second byte is 0)
}
