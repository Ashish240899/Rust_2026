# 🚀 Phase 02: Rust Database API (Raw Prototype)

## 🏗️ Architecture
This project is a low-level, custom-built HTTP API server that bridges the gap between a relational database (SQLite) and web clients, strictly using Rust's standard networking library and zero heavy web frameworks (like Actix or Axum). 

- **Backend Protocol:** TCP/HTTP.
- **Database:** SQLite (`rusqlite` crate).
- **Data Serialization:** JSON (`serde` & `serde_json` crates).
- **Core Engine:** Custom incoming connection loop parsing raw HTTP `GET` requests.

## 🛠️ Traits & Generics Used
- `#[derive(Serialize)]`: Automatically implements the Serde `Serialize` trait on the `User` struct, converting in-memory Rust data directly into a JSON stream.
- `Vec<User>`: A dynamically sized generic collection used to hold iterations of database rows mapped into Rust Structs.
- `std::io::{Read, Write}`: Core traits utilized for buffer streaming via raw TCP streams.

## 🌍 Real-World Utility
This architecture forms the raw foundational logic of modern Microservices. By manually mapping SQLite rows `(|row|)` and formatting raw HTTP response headers (`HTTP/1.1 200 OK \r\n Content-Type: application/json \r\n\r\n`), this prototype demonstrates the exact underlying mechanics of how large-scale enterprise APIs (like Twitter or Discord) fetch data from a database and serve it as JSON to mobile and web frontends.

> **Note:** This is a WIP raw prototype. Strict crash-proofing and error handling (`match` blocks) are scheduled for the next commit.