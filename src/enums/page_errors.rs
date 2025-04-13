use std::default;

#[derive(Debug)]
pub enum PageError {
    // Occurs when slot does not exist in slot table
    SlotNotFound,
}
