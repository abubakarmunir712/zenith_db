use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use crate::storage::io::file_io::IOEngine;

pub fn initialize_listener() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind");
    let connection_counter = Arc::new(AtomicUsize::new(0));

    println!("Server listening on port 7878...");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };

        let counter = Arc::clone(&connection_counter);
        if counter.load(Ordering::SeqCst) >= 5 {
            writeln!(&stream, "Error: Max connections limit reached").ok();
            continue;
        }

        counter.fetch_add(1, Ordering::SeqCst);
        thread::spawn(move || handle_client(stream, counter));
    }
}

fn parse_connection_string(conn_str: &str) -> Option<String> {
    let db_name = conn_str.trim();
    if !db_name.is_empty() {
        if IOEngine::db_exists(db_name) {
            return Some(db_name.to_string());
        } else {
            return None;
        }
    } else {
        None
    }
}

fn handle_client(mut stream: TcpStream, counter: Arc<AtomicUsize>) {
    let mut buffer = [0u8; 1024 * 32];

    // Read initial connection string
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) | Err(_) => {
            counter.fetch_sub(1, Ordering::SeqCst);
            return;
        }
        Ok(n) => n,
    };

    let conn_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let db_name = match parse_connection_string(&conn_str) {
        Some(db) => {
            writeln!(stream, "Connected to database '{}'", db).ok();
            println!("Client connected to DB: {}", db);
            db
        }
        None => {
            writeln!(stream, "ERROR: Database not found").ok();
            counter.fetch_sub(1, Ordering::SeqCst);
            return;
        }
    };

    // Keep connection alive and handle queries
    loop {
        let read = match stream.read(&mut buffer) {
            Ok(0) => break, // disconnected
            Ok(n) => n,
            Err(_) => break,
        };

        let query = String::from_utf8_lossy(&buffer[..read]).trim().to_string();

        if query.is_empty() {
            continue;
        }

        if query.trim() == "db" {
            writeln!(stream, "{db_name}").ok();
        }

        println!("Query from [{}]: {}", db_name, query);

        // 👉 Place to call actual query parser

        writeln!(stream, "Query received on '{}': {}", db_name, query).ok();
    }

    counter.fetch_sub(1, Ordering::SeqCst);
}
