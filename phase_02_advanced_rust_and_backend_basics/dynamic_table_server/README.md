# ⚙️ Dynamic Math Engine (Rust Web Server) - Phase 02

## 📌 Project Overview
A dynamic, low-level HTTP Web Server built entirely from scratch using Rust's `std::net`. This project upgrades the static server into a dynamic engine that intercepts user input via HTML forms, extracts URL query parameters (`?num=x`), processes the math logic in the backend, and renders a dynamic HTML multiplication table on the fly.

## 🧠 Core Architecture & Tech Stack
* **Language:** Rust (The Giant)
* **Dynamic Routing:** Strict `if-else` routing to serve distinct HTML components (Forms, Tables, 400 Bad Request, 404 Not Found).
* **String Manipulation:** Advanced usage of `.split()`, `.nth()`, and `.next()` to surgically extract raw data from HTTP GET requests.
* **Type Casting & Crash-Proofing:** Safely parsing strings to `i32` integers using `.parse::<i32>()` and gracefully handling invalid inputs with `.unwrap_or(0)` to guarantee zero server panics.
* **Dynamic HTML Injector:** Utilizing Rust's `format!` macro to dynamically embed backend variables into frontend HTML templates.

## 💻 Usage
```bash
cargo run
# 1. Open [http://127.0.0.1:8080](http://127.0.0.1:8080) in Chrome.
# 2. Enter any valid number in the form and hit 'Generate Table'.
# 3. Observe the dynamic math calculation rendered via HTML.
# 4. Try inputting strings (e.g., "Ashish") to test the 400 Bad Request fallback.