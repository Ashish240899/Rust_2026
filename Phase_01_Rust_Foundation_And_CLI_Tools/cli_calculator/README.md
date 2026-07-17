# 🧮 CLI Calculator (Rust)
## Phase 01: Foundation

An enterprise-grade, memory-safe CLI calculator built using advanced Rust structures (Structs & Enums).

### 🛠️ Core Architecture & Traits
* **Data Modeling:** Implemented custom `enum Operation` and `struct Calculator` for clean, scalable, and idiomatic Rust architecture.
* **Precision Math:** Utilized `f64` to support high-precision floating-point arithmetic.
* **Crash-Proof Design:** - Strict pattern matching (`match`) on user input to handle invalid strings gracefully without panicking.
  - Hardcoded Zero-Division shield (`if num2 == 0.0`) to prevent runtime mathematical exceptions.
* **UI/UX:** Enhanced with the `colored` crate for a visual, hacker-style terminal output.