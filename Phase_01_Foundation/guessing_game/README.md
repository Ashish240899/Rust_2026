# 🎯 Rust CLI Guessing Game (Crash-Proof)

An interactive, robust, and memory-safe Command Line Interface (CLI) Guessing Game built entirely in Rust. This is Phase 1 of my hardcore Rust Backend Development portfolio.

## 🧠 Core Architecture
This project demonstrates control flow and error handling at a production level:
* **Random Number Generation (`rand` crate):** Utilizing external crates to securely generate random numbers.
* **Infinite Game Loop (`loop`):** Keeps the game running until the precise winning condition is met.
* **Strict Type Casting & Shadowing:** Converting String inputs into `u32` integers safely using variable shadowing.
* **Pattern Matching (`match` & `Ordering`):** Used to compare variables (`Less`, `Greater`, `Equal`) efficiently.
* **Crash-Proof Input Validation:** Implemented `match` on `.parse()` with a `continue` statement to gracefully catch non-numerical inputs without exiting the game loop.

## 💻 Tech Stack
* **Language:** Rust
* **Development Environment:** Windows 11 Pro, Visual Studio Code

## 🚀 How to Run
1. Clone this repository to your local machine.
2. Open the terminal and navigate to the project directory.
3. Run the following command:
   ```bash
   cargo run
   