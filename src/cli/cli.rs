use std::io::{self, BufRead, Write};
use std::sync::Arc;

use crate::enums::types::res_type::ResType;
use crate::executor::executor::Executor;
use crate::parser::parser::Parser;
use crate::storage::io::file_io::IOEngine;

pub fn start_cli(executor: Arc<Executor>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Initial DB connection
    println!("Enter database name to connect:");
    input.clear();
    stdin.lock().read_line(&mut input).unwrap();
    let conn_str = input.trim();

    let db_name = match parse_connection_string(conn_str) {
        Some(db) => {
            println!("Connected to database '{}'", db);
            db
        }
        None => {
            println!("ERROR: Database '{}' not found", conn_str);
            return;
        }
    };

    // Query loop
    loop {
        print!(">> ");
        stdout.flush().unwrap();
        input.clear();
        if stdin.lock().read_line(&mut input).is_err() {
            println!("Error reading input. Exiting.");
            break;
        }

        let query = input.trim();
        if query.is_empty() {
            continue;
        }

        if query.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        if query == "db" {
            println!("{}", db_name);
            continue;
        }

        println!("Received query: '{}' on DB: {}", query, db_name);

        let result = Parser::parse_query(query, &db_name, executor.clone());
        let response = match result {
            ResType::Error(s) => format!("Error: {}", s),
            ResType::Success(s) => format!("Success: {}", s),
            ResType::View(v) => v.display(),
        };

        println!("{}", response);
    }
}

fn parse_connection_string(conn_str: &str) -> Option<String> {
    let db_name = conn_str.trim();
    if !db_name.is_empty() && IOEngine::db_exists(db_name) {
        Some(db_name.to_string())
    } else {
        None
    }
}
