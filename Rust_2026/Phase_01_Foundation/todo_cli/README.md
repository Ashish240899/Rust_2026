# 📝 Rust CLI To-Do App (Crash-Proof)

A fast, dynamic, and memory-safe Command Line Interface (CLI) To-Do List application built in Rust. This is Phase 1 of my hardcore Rust Backend Development portfolio.

## 🧠 Core Architecture
This project demonstrates dynamic memory allocation and robust state management:
* **Dynamic Vectors (`Vec<String>`):** Utilized Rust's heap-allocated Vector to dynamically grow and shrink the task list in real-time.
* **Iterators & Enumerate (`.iter().enumerate()`):** Cleanly mapped indices to tasks for a user-friendly UI experience.
* **Index Guard & Bounds Checking:** Protected the `.remove()` method with strict logical boundaries (`> 0 && <= len`) to prevent Out-Of-Bounds memory panics.
* **Crash-Proof Parsing:** Engineered a `match` expression on input parsing to gracefully catch `Err` strings and loop back using `continue` instead of crashing.
* **Graceful Exit:** Replaced abrupt panics with a safe `break` logic for a clean program termination.

## 💻 Tech Stack
* **Language:** Rust
* **Development Environment:** Windows 11 Pro, Visual Studio Code

## 🚀 How to Run
1. Clone this repository to your local machine.
2. Open the terminal and navigate to the project directory.
3. Run the following command:
   ```bash
   cargo run
   