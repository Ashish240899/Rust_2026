# 🚀 Rust CLI Calculator (Crash-Proof)

A fast, memory-safe, and crash-proof Command Line Interface (CLI) Calculator built entirely in Rust. This is Phase 1 of my hardcore Rust Backend Development journey.

## 🧠 Core Architecture
Instead of using basic variables, this project is engineered using advanced Rust concepts to ensure scalability and strict type safety:
* **Structs (`Calculator`)**: To encapsulate the operands and operations into a single clean entity.
* **Enums (`Operation`)**: To define strict mathematical operations (Add, Sub, Mul, Div) and prevent invalid inputs.
* **Pattern Matching (`match`)**: Used for routing operations and handling invalid characters without crashing.
* **Error Handling**: Custom `Ok` and `Err` match arms to gracefully handle non-numerical inputs and Division-by-Zero traps, ensuring 100% uptime.

## 🛠️ Features
- Addition (`+` or `Add`)
- Subtraction (`-` or `Sub`)
- Multiplication (`*` or `Mul`)
- Division (`/` or `Div`) (Protected against Zero-Division)
- **Crash-Proof Input Validation:** Gracefully catches string inputs when numbers are expected.

## 💻 Tech Stack
* **Language:** Rust
* **Development Environment:** Windows 11 Pro, Visual Studio Code

## 🚀 How to Run
1. Clone this repository to your local machine.
2. Open the terminal and navigate to the project directory.
3. Run the following command:
   ```bash
   cargo run
   