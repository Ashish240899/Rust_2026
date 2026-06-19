# 🎮 Rust CLI Tic-Tac-Toe (Crash-Proof)

A fully functional, memory-safe, and zero-crash Command Line Interface (CLI) Tic-Tac-Toe game built entirely in Rust. The final boss of my Phase 1 Rust Backend Development portfolio.

## 🧠 Core Architecture
This project demonstrates advanced 2D array manipulations, mathematical logic, and game state management:
* **2D Matrices (`[[char; 3]; 3]`):** Engineered the game board using fixed-size 2D arrays for strict memory safety.
* **Mathematical Index Mapping:** Utilized modulo and division operators `(input-1)/3` and `(input-1)%3` to dynamically map 1D user inputs (1-9) to 2D grid coordinates.
* **Buffer Clearing (`input.clear()`):** Prevented memory appending bugs in the `read_line` loop by aggressively clearing the String buffer before each turn.
* **Smart Pattern Matching:** Combined `.parse()` with a guard clause `if no >= 1 && no <= 9` to validate both the data type and the numerical boundaries in a single clean sweep.
* **Anti-Overwrite Protection:** Handled Edge Cases to prevent users from overriding already played 'X' or 'O' positions.

## 💻 Tech Stack
* **Language:** Rust
* **Development Environment:** Windows 11 Pro, Visual Studio Code

## 🚀 How to Run
1. Clone this repository to your local machine.
2. Open the terminal and navigate to the project directory.
3. Run the following command:
   ```bash
   cargo run