# 🚀 Pure Rust HTTP Web Calculator

## 📌 Enterprise Blueprint
A 100% dependency-free, memory-safe backend web server and HTTP parser built entirely from scratch using Rust's standard library (`std::net::TcpListener`). This project strictly avoids external frontend files, relying purely on raw Rust strings, Unicode, and backend rendering to maintain absolute focus on the core logic.

## 🧠 Core Architecture & Traits
*   **Raw TCP Parsing:** Manually extracts query parameters (`n1`, `n2`, `op`) from raw HTTP GET requests.
*   **Crash-Proof Engine:** Strict edge-case handling using `.unwrap_or()`. Prevents panic loops from invalid URL routing or empty parameters.
*   **Zero-Division Shield:** Hardcoded logic to intercept and block `Division by Zero` and `Modulo by Zero` operations, returning a graceful `400 BAD REQUEST` instead of yielding `NaN` or panicking.
*   **Dynamic UI Injection:** Renders an interactive, emoji-based HTML UI directly from the Rust format macro string.

## ⚙️ The Math Engine (f64 Operations)
Supports high-precision floating-point arithmetic:
- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)
- Remainder/Modulo (`%`)
- Exponential Power (`xⁿ` using `.powf()`)