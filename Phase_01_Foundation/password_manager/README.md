# 🔐 CLI Password Manager V2.0 (Rust)
## Phase 02: Advanced Backend Basics

An interactive, memory-safe Command Line Interface (CLI) Password Manager featuring complete CRUD operations.

### 🛠️ Core Architecture & Traits
* **Dynamic State Management:** Utilized standard library `Vec<String>` for dynamic heap allocation of IDs and Credentials.
* **Memory-Safe Error Handling:** Implemented `match` control flow for `Result` (parsing user inputs) and `Option` (searching vector indices), ensuring a zero-panic runtime.
* **CRUD Implementation:** Engineered full Create, Read, Update, and Delete functionalities using iterative closures (`.iter().position()`) and safe index removal (`.remove()`).