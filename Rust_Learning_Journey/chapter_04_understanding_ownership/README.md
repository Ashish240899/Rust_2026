# Chapter 04: Understanding Ownership 🧠💥

This repository contains 10 hardcore Rust programs strictly verifying the concepts of Ownership, Borrowing, Slices, and Memory Safety. Built as Phase 01 of the 1.5Cr+ CTO vision. Zero Garbage Collector, Zero Dangling Pointers.

## 🚀 The Arsenal
* **Ownership Rules:** Move, Copy, and Clone traits.
* **Borrowing:** Immutable (`&`) vs Mutable (`&mut`) references.
* **The Borrow Checker:** Preventing Data Races and Double Free errors.
* **Slices:** `&str` and `&[i32]` for safe contiguous memory referencing.

## 📂 Project Breakdowns
1. **01_ownership_transfer_tracker**: Demonstrates the 'Move' trait and how old owners are invalidated to prevent Double Free errors.
2. **02_heap_clone_machine**: Utilizes `.clone()` to perform deep copies of heap-allocated data.
3. **03_stack_copy_vip**: Verifies the 'Copy' trait for fixed-size stack variables (integers, booleans).
4. **04_function_move_trap**: Shows how passing variables to functions transfers ownership and triggers the `drop` function.
5. **05_return_ownership_rescue**: Recovers ownership from a function scope via return values.
6. **06_immutable_borrow_lens**: Passes data as read-only references (`&`) to avoid transferring ownership.
7. **07_mutable_borrow_modifier**: Mutates original heap data through a single, exclusive mutable reference (`&mut`).
8. **08_data_race_prevention**: Uses manual scoping `{}` to safely manage multiple mutable borrows over time.
9. **09_first_word_slicer**: Implements the official Rust slice algorithm to extract words memory-safely (`&str`).
10. **10_array_slice_scanner**: Extends the slice concept to fixed-size stack arrays (`&[i32]`).