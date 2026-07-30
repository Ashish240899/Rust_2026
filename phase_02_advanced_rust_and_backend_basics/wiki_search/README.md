# 🚀 Interactive Wiki Search Engine (Rust)

## 📌 Project Overview
An ultra-fast, memory-safe Interactive Command Line Interface (CLI) application built in **Rust** that fetches real-time summaries from the Wikipedia API. This project demonstrates Phase 02 Advanced Rust concepts, specifically focusing on standard I/O streams, asynchronous network requests, robust error handling, and safe JSON deserialization.

## ⚙️ Core Architecture & Tech Stack
* **Language:** Rust (Strictly Safe Rust)
* **User Input:** `std::io::stdin` for interactive terminal querying.
* **Concurrency:** `tokio` (Asynchronous runtime for non-blocking network requests).
* **Networking:** `reqwest` (HTTP Client handling custom headers, GET requests, and TCP/IP flow).
* **Data Parsing:** `serde_json` (Safely deserializing dynamic JSON payloads into Rust types).
* **Memory Safety:** Implemented exact pattern matching (`match`), `if let`, and `Result` paradigms to ensure zero unhandled panics, even on empty inputs.

## 🧠 Logic Flow & Data Pipeline
1. **Interactive Prompt:** Pauses execution and captures raw user input via `io::stdin().read_line()`.
2. **Input Sanitization:** Trims whitespace and safely matches against empty strings (`""`) to prevent invalid API calls.
3. **Endpoint 1 (Index Validation):** Hits Wikipedia's `w/api.php` endpoint to validate the exact Wikipedia title.
4. **Data Mutation:** Cleans raw JSON text by replacing spaces with underscores to generate a valid URL-encoded string.
5. **Endpoint 2 (Data Extraction):** Hits the `api/rest_v1/page/summary/` endpoint to fetch the targeted article's summary.
6. **Pattern Matching:** Safely unwraps the `["extract"]` JSON node and renders the data cleanly to the standard output.

## 💻 Usage
```bash
cargo run
# The program will interactively prompt: "Search On Wikipedia?"