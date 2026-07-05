# 🎯 CLI Guessing Game (Rust)
## Phase 01: Foundation

An advanced, memory-safe number guessing game featuring dynamic difficulty levels and idiomatic Rust architecture.

### 🛠️ Core Architecture & Traits
* **Modular Design:** Segregated menu routing (`main`) and core game loop (`get_guess`) for maximum code reusability.
* **Idiomatic Control Flow:** Implemented `std::cmp::Ordering` with `match` for highly optimized, exhaustive number comparisons.
* **Dynamic Boundaries:** RNG (Random Number Generator) scales dynamically based on user-selected difficulty thresholds (50, 100, 1000).
* **Bulletproof Inputs:** Strict pattern matching and mathematical boundary checking to trap out-of-range anomalies and non-integer crashes.