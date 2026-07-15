# 🚀 CLI Wiki Search Engine (Rust)

## 📌 Project Overview
An ultra-fast, memory-safe Command Line Interface (CLI) application built in **Rust** that fetches real-time summaries from the Wikipedia API. This project strictly demonstrates Phase 02 Advanced Rust concepts including asynchronous network requests, robust error handling, and safe JSON deserialization. It is designed to be the foundational API module for future interactive tools.

## ⚙️ Core Architecture & Tech Stack
* **Language:** Rust (Strictly Safe Rust)
* **Concurrency:** `tokio` (Asynchronous runtime for non-blocking network requests)
* **Networking:** `reqwest` (HTTP Client handling headers, GET requests, and TCP/IP flow)
* **Data Parsing:** `serde_json` (Safely deserializing dynamic JSON payloads into Rust types)
* **Memory Safety:** Implemented `if let`, `Option`, and `Result` paradigms (e.g., `unwrap_or("")`) to ensure zero unhandled panics.

## 🧠 Logic Flow & Data Pipeline
1. **Direct Argument Parsing:** Captures user input efficiently using optimized `std::env::args`.
2. **Endpoint 1 (Index Validation):** Hits Wikipedia's `w/api.php` endpoint to validate the `search_value` and extract the precise title.
3. **Data Mutation:** Cleans raw JSON text by replacing spaces with underscores to generate a valid `correct_value` for URL encoding.
4. **Endpoint 2 (Data Extraction):** Hits the `api/rest_v1/page/summary/` endpoint to fetch the targeted article's summary.
5. **Pattern Matching:** Safely unwraps the `["extract"]` JSON node and renders the data cleanly to the standard output.

## 💻 Usage
```bash
cargo run -- <search_value>
# Example: cargo run -- India