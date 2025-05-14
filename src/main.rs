mod configs;
mod enums;
mod indexing;
mod oid;
mod storage;
mod types;
mod utils;
// pub mod parser;

use ZenithDB::{networking::handler::initialize_listener};



fn main() {
    initialize_listener();
}
