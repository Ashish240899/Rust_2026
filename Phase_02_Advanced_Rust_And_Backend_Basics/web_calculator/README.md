# 🚀 Rust Web Calculator (Enterprise Edition)

## 📌 Overview
An advanced, memory-safe HTTP Web Calculator built entirely from scratch in Rust. This updated iteration replaces all fragile `.unwrap()` calls with strict `.expect()` error handling, ensuring robust terminal logging and zero silent panics.

## 🧠 Core Architecture & Upgrades
* **Custom Error Handling:** Integrated `.expect()` across TCP listeners and byte readers to provide granular, developer-friendly crash reports.
* **Advanced Math Engine:** Added modular arithmetic (Remainder) and Exponential (Power) operations.
* **Hardcore Typecasting:** Implemented explicit casting (`n1 as i32`, `n2 as u32`) to satisfy Rust's strict `.pow()` trait requirements directly within the custom HTTP parser block.
* **Crash-Proof Logic:** Sustains the `unwrap_or()` shield against invalid text inputs and URL manipulation.