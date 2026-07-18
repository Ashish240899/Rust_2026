# 🚀 Rust Web Calculator (Crash-Proof Backend)

## 📌 Overview
A lightning-fast, zero-dependency, and 100% memory-safe HTTP Web Calculator built entirely from scratch in Rust. This project bypasses external crates (like Actix or Rocket) to demonstrate a raw, deep-level understanding of TCP Sockets, HTTP Protocol parsing, and hardcore memory-safe edge case handling.

## 🧠 Core Architecture & Logic
* **TCP Networking:** Utilizes `std::net::TcpListener` for raw socket binding on `127.0.0.1:8080` and robust client connection handling via byte streams.
* **Custom HTTP Parser:** Implements a manual HTTP GET request router to accurately parse URL query parameters (`?n1=x&n2=y&op=z`) using precise string slicing (`.split()`) without data loss.
* **Memory Safety First:** 100% panics eliminated. Employs Rust's powerful `.unwrap_or()` for secure fallback defaults, ensuring the server never crashes on invalid user string inputs, alphabets, or missing operators.
* **Zero-Division Shield:** Features a hardcoded algorithmic interceptor (`if n2 == 0.0`) that catches zero-division math errors and fires a strict `400 BAD REQUEST` HTTP header directly to the browser.
* **Pattern Matching:** Uses Rust's exhaustive `match` control flow for dynamic mathematical routing (`add`, `sub`, `mul`, `div`), including a catch-all wildcard (`_`) to destroy unauthorized parameters.

## 🛠️ Real-World Utility
This project serves as a foundational blueprint for building enterprise-grade microservices, REST APIs, and raw web servers. It proves that backend logic can be handled securely at the lowest level without relying on heavy, bloated frameworks.