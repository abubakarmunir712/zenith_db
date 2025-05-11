use storage::buffer::catalog_buffer::CatalogBuffer;
use storage::{buffer::page_buffer::PageBuffer, catalog::catalog_manager::CatalogManager};

mod configs;
mod enums;
mod indexing;
mod oid;
mod storage;
mod types;
mod utils;

fn main() {
    // // Buffers Initialization
    // let p_buff = PageBuffer::new();
    // let c_buff = CatalogBuffer::new();

    // // Manager Initialization
    // let c_mngr = CatalogManager {
    //     catlog_buffer: c_buff,
    // };
}
