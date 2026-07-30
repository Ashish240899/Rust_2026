# 🚀 Phase 02: Rust Database API (Crash-Proof Edition)

## 🏗️ Architecture
This project is a low-level, custom-built HTTP API server that bridges the gap between a relational database (SQLite) and web clients. It strictly utilizes Rust's standard networking library, avoiding heavy web frameworks (like Actix or Axum) to master the core mechanics of backend architecture.

- **Backend Protocol:** TCP/HTTP.
- **Database:** SQLite (`rusqlite` crate).
- **Data Serialization:** JSON (`serde` & `serde_json` crates).
- **Core Engine:** Custom incoming connection loop parsing raw HTTP `GET` requests.

## 🛡️ Enterprise Crash-Proofing (Zero Panic Policy)
This API is engineered to be 100% memory-safe and crash-proof. 
- **Server-Level Isolation:** Database initialization and Port binding errors are handled using strict `match` blocks triggering graceful shutdowns via `std::process::exit(1)`.
- **Client-Level Isolation:** Network stream errors (e.g., failed buffer reads or dropped connections) within the main server loop utilize the `continue` keyword. This ensures a single faulty client cannot crash the entire host server, maintaining 100% uptime for other active connections.

## 🛠️ Traits & Generics Used
- `#[derive(Serialize)]`: Automatically implements the Serde `Serialize` trait on the `User` struct, converting in-memory Rust data directly into a JSON stream.
- `Vec<User>`: A dynamically sized generic collection used to hold iterations of database rows mapped into Rust Structs.
- `std::io::{Read, Write}`: Core traits utilized for buffer streaming via raw TCP streams.

## 🌍 Real-World Utility
This architecture forms the raw foundational logic of modern Microservices. By manually mapping SQLite rows `(|row|)` and formatting raw HTTP response headers, this prototype demonstrates the exact underlying mechanics of how large-scale enterprise APIs fetch data from a database and serve it as JSON to frontends without relying on external magic.