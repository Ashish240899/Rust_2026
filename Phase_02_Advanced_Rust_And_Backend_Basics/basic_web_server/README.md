# 🕸️ Basic HTTP Web Server (Rust)

## 📌 Project Overview
A fully functional, low-level, zero-dependency HTTP Web Server built entirely from scratch using Rust's standard library (`std::net`). This project demonstrates how computers communicate over the internet at the foundational TCP/IP level, completely bypassing modern high-level frameworks.

## ⚙️ Core Architecture & Tech Stack
* **Language:** Rust
* **Network Protocol:** Transmission Control Protocol (TCP) via `TcpListener` & `TcpStream`.
* **Data Flow:** Captures incoming browser requests into a 1024-byte memory `buffer`, processes the raw bytes, and flushes a valid `HTTP/1.1 200 OK` response directly to the client socket.
* **Resilience:** Implemented strict error handling (`.expect()`) at every point of failure (Port Binding, Connection Acceptance, Data Reading, and Stream Flushing) to prevent server panics.

## 💻 Usage
```bash
cargo run
# The server will bind to port 8080.
# Open Google Chrome and navigate to [http://127.0.0.1:8080](http://127.0.0.1:8080) to see the live rendering.