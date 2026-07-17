# 📝 To-Do CLI (Rust)
## Phase 01: Foundation

A 100% memory-safe, colorized terminal To-Do list application built in Rust using dynamic Vectors.

### 🛠️ Core Architecture & Traits
* **Dynamic Data Structures:** Utilized Rust's `Vec<String>` for efficient heap-allocated task storage and dynamic resizing.
* **Memory Safety:** Enforced strict index boundary checks (`delete > 0 && delete <= to_do.len()`) to prevent Out-Of-Bounds panics during task removal.
* **Error Handling:** Leveraged `match` with `parse::<usize>()` to cleanly trap and handle invalid non-integer inputs.
* **UI/UX:** Integrated the `colored` crate for an intuitive, hacker-style terminal interface.