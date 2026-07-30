# 🕸️ Advanced HTTP Web Server (Rust) - Phase 02

## 📌 Project Overview
An upgraded, low-level HTTP Web Server built entirely with Rust's `std::net`. This version introduces custom **Routing** and **Logging** mechanisms directly from scratch, intercepting raw TCP byte streams and decoding browser signatures without relying on any external frameworks.

## ⚙️ Core Architecture & Tech Stack
* **Language:** Rust (The Giant)
* **Routing Engine:** Implemented strict HTTP GET request parsing (`GET / HTTP/1.1`) to serve 200 OK responses, and a robust fallback to generate `404 NOT FOUND` headers for invalid endpoints.
* **Logging System:** Real-time decoding of incoming browser requests to monitor User-Agent, OS, and client metadata directly in the terminal.
* **Memory Safety & Crash-Proofing:** Utilized `String::from_utf8_lossy()` to gracefully handle malformed TCP byte streams without panicking the server. 
* **Borrowing Rules:** Strict ownership enforced using slice references `&buffer[..]` combined with `.to_string()` for zero-cost data decoding and memory isolation.

## 💻 Usage
```bash
cargo run
# The server will bind to 127.0.0.1:8080.
# 1. Check the terminal for live HTTP request logs (CCTV).
# 2. Visit [http://127.0.0.1:8080](http://127.0.0.1:8080) to test the 200 OK Home Page.
# 3. Visit [http://127.0.0.1:8080/hacker](http://127.0.0.1:8080/hacker) to test the strict 404 Router fallback.