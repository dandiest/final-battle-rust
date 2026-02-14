<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Stable-green.svg" />
</p>

<h1 align="center">⚔️ Rust Final Battle - Multi-Character Battle System</h1>

<p align="center">
  A robust Command Line Interface (CLI) engine demonstrating advanced Rust patterns: Enums with data, custom Types, and synchronized loops.
</p>

---

## 🎓 Educational Disclaimer
This repository is part of my **Personal Apprenticeship** and learning journey with the Rust programming language. 
* **Purpose**: This code is strictly for educational purposes. It serves as a practical sandbox to master Rust's ownership system, memory safety, and type-driven development.
* **Evolution**: As I progress in my apprenticeship, this code will be refactored to implement more advanced concepts (Error Handling, Smart Pointers, Concurrency).
* **Feedback**: Constructive feedback is welcome as I work towards becoming a proficient Rust developer!

## 🌟 Features
* **Custom Power System**: Implemented a `Powers` Enum (`Ice`, `Fire`) to categorize character abilities.
* **Synchronized States**: Manages multiple `Character` instances (Matthew & Luke) through a shared charging cycle.
* **Smart Printing**: Uses Rust's `Debug` trait and custom `impl` methods for clean, informative console output.
* **State Logic**: Automatic transition from `Charging` to `Attacking` once the energy threshold (100%) is reached.

## 🛠️ Technical Deep Dive
* **Associated Data**: The `Actions` Enum doesn't just name a state; it carries values like `u32` for percentages and `String` for attack names.
* **Pattern Matching**: Exhaustive `match` blocks ensure that every possible character state is handled safely by the compiler.
* **Memory Safety**: leveraged references (`&self`) to prevent unnecessary data cloning during display cycles.



---

## 🚀 How to Run
1. Clone the repository.
2. Ensure you have [Rust](https://www.rust-lang.org/) installed.
3. Run the following command in your terminal:
   ```bash
   cargo run
   
---

## ⚖️ License & Copyright
Copyright (c) 2026 **[dandiest]**

This project is licensed under the **MIT License**. 

*You are free to use, study, and modify this code for educational purposes. Please provide attribution if you use significant portions of this logic in your own learning journey.*
