# Rust README (2024 goals)

Welcome to my Rust learning journey! 🚀 This README is like a storybook about my Rust journey. It'll keep growing as I learn more, adding new things I discover in the world of Rust. Throughout the year, I'll share my progress as I become proficient in Rust. 📖✨

## Table of Contents

1. [Ownership](#ownership)
   - [Ownership Rules](#ownership-rules)
   - [Ownership Transfers](#ownership-transfers)
   - [Borrowing](#borrowing)

2. [Lifetimes](#lifetimes)
   - [Understanding Lifetimes](#understanding-lifetimes)
   - [Lifetime Annotations](#lifetime-annotations)
   - [Lifetime Elision](#lifetime-elision)
   - [Lifetime Bounds](#lifetime-bounds)

**Will be adding more daily**

## Ownership

Ownership is one of Rust's most distinctive features. It allows Rust to achieve memory safety without a garbage collector.

### Ownership Rules

- **Each value in Rust has a variable that is its "owner."**
- **There can only be one owner at a time.**
- **When the owner goes out of scope, the value is dropped.**

### Ownership Transfers

Ownership can be transferred using moves. When a value is assigned to another variable or passed as a function argument, ownership is transferred.

### Borrowing

Borrowing allows a function to take references to values without taking ownership. Borrowing can be mutable or immutable.

---

## Lifetimes

Lifetimes ensure that references in your code are valid. They prevent dangling references and contribute to Rust's memory safety.

### Understanding Lifetimes

Lifetimes are a way of expressing the scope during which references are valid. They are denoted by single-quote characters, like `'a`.

### Lifetime Annotations

In Rust, lifetime annotations are used to describe the relationship between the lifetimes of function parameters and return values.

### Lifetime Elision

Rust has lifetime elision rules that allow the omission of explicit lifetime annotations in many common cases, making code more concise.

### Lifetime Bounds

Lifetimes can be bounded to specific scopes using lifetime bounds, ensuring references are valid for a certain duration.

---


