// file_io.rs - Handles low-level file I/O operations for the storage engine.
//
// This file contains essential functions for reading and writing pages 
// to disk, ensuring efficient and reliable data persistence. It provides 
// abstractions over system file operations, including handling page-aligned 
// reads/writes, flushing data, and managing file descriptors.
//
// File I/O is a critical component of the storage layer, interacting 
// closely with the buffer pool and WAL (Write-Ahead Logging) system.
//