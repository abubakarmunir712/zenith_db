mod configs;
mod enums;
mod executor;
mod indexing;
mod cli;
mod oid;
mod parser;
mod storage;
mod types;
mod utils;

use crate::{
    executor::executor::Executor,
    // networking::handler::initialize_listener,
    storage::io::file_io::IOEngine,
};
use cli::cli::start_cli;

fn main() {
    if !IOEngine::db_exists("root") {
        IOEngine::create_db("root").unwrap();
    }
    let executor = Executor::new();
    // initialize_listener(executor);
    start_cli(executor);
}
