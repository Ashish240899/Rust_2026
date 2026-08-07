# Chapter 8: Common Collections (Vector, String, HashMap)

This repository contains 10 hardcore, CTO-level Rust programs built to master Heap-allocated memory structures, UTF-8 safety, and advanced hashing mechanisms. Zero crashes, zero panics, 100% memory safety.

## 🚀 Arsenal Deployed
*   **Vectors (`Vec<T>`):** Dynamic sizing, Enum encapsulation, and safe retrieval using `Option`.
*   **Strings (`String` & `&str`):** UTF-8 byte boundary management, `.chars()` safety, and string concatenations.
*   **Hash Maps (`HashMap<K, V>`):** O(1) lookups, Ownership handling, `.entry()`, and `.or_insert()` updating algorithms.

## 🛠️ The 10 Missions
1.  `01_vector_stats_analyzer`: Calculates Mean, Median, and Mode using Vectors and Hash Maps.
2.  `02_pig_latin_converter`: Safely manipulates String characters based on Vowel/Consonant rules.
3.  `03_employee_department_manager`: A CLI-based Hash Map linking String keys to Vector values.
4.  `04_safe_vector_retrieval`: Demonstrates `match` and `.get()` to prevent Out-of-Bounds panics.
5.  `05_multi_type_vector`: Uses Enums to bypass Vector's homogenous data type constraint.
6.  `06_string_slicing_safeguard`: Extracts Unicode characters without causing byte-slicing panics.
7.  `07_word_frequency_counter`: Implements `or_insert()` to build a high-speed word counter.
8.  `08_team_score_updater`: Proves the difference between overwriting and conditional inserting.
9.  `09_invisible_byte_detector`: Uses `.escape_debug()` to reveal hidden terminal control codes.
10. `10_mega_collection_database`: A unified project combining Vec, String, and HashMap into a mini-DB.