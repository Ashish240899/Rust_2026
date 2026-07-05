# 🎮 Tic-Tac-Toe (Rust CLI)
## Phase 01: Foundation

A 100% memory-safe, crash-proof, and colorized terminal Tic-Tac-Toe game built in Rust.

### 🛠️ Core Architecture & Traits
* **Memory Safety:** Strict boundary checking (`num >= 1 && num <= 9`) to completely eliminate Array Out-Of-Bounds panics.
* **Error Handling:** Implemented `match` and `parse::<usize>()` to cleanly trap and handle invalid string inputs without crashing.
* **UI/UX:** Integrated the `colored` crate for dynamic `X` (Red) and `O` (Green) terminal rendering.
* **DRY Codebase:** Modularized grid rendering via `print_tab` and `print_pro` helper functions for a clean `main` loop.