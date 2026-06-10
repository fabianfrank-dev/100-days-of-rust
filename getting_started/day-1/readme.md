c// ... existing code ...
# Day 1: Basics (Hello World, User Input, Calculator)

This module covers fundamental concepts learned during **1**. The primary goal was to solidify understanding of Rust features required for this stage of learning.

## 🎯 Projects Implemented
- **Hello World**: Successfully printed the initial greeting to the console, understanding Rust's entry point (`main`).
- **User Input**: Built functionality to read string and number inputs from the standard input stream using `std::io`.
- **Calculator**: Created a basic command-line application capable of performing core arithmetic operations (+, -, \*, /).

## ✨ Key Learnings & Concepts
✅ **Basic I/O Operations**: Mastering how to interact with the operating system console, specifically reading user text and numbers (`.read_line()`).
✅ **Variable Mutability**: Understanding when variables need to be declared as mutable (`let mut`) and when they are immutable by default.
✅ **Arithmetic & Type Casting**: Implementing mathematical logic while correctly handling type conversions (e.g., from `String` input to numeric types like `i32`).

## 🚧 Challenges & Next Steps
- **Error Handling in Input**: The most significant challenge was gracefully handling potential runtime errors, such as the user entering text when a number was expected. This requires using `.expect()` or `match` on parsing functions.
- **Structuring Code**: Learning to separate concerns into different modules (like treating I/O logic separately from calculation logic).

