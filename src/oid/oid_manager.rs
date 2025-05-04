/// Generates the lowest available OID based on a bitmask.
/// Returns None if all OIDs are used.
///
/// # Arguments
/// * `bitmask` - a slice of u8 where each bit represents usage (1 = used, 0 = free)
pub fn generate_oid(bitmask: &[u8]) -> Option<usize> {
    let size = bitmask.len() * 8;
    for (byte_index, byte) in bitmask.iter().enumerate() {
        for bit in 0..8 {
            let global_bit = byte_index * 8 + bit;
            if global_bit >= size {
                break;
            }
            if byte & (1 << (7 - bit)) == 0 {
                return Some(global_bit + 1);
            }
        }
    }
    None
}

pub fn delete_oid(bitmask: &mut Vec<u8>, value: u16) {
    let index= (value as usize - 1) / 8; 
    let bit_pos = (value - 1) % 8;
    bitmask[index] |= 1 << (7 - bit_pos);
}

pub fn undelete_oid(bitmask: &mut Vec<u8>, value: u16) {
    let index = (value as usize - 1) / 8;
    let bit_pos = (value - 1) % 8; 
    bitmask[index] &= !(1 << (7 - bit_pos));
}
