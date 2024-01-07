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
   - [Lifetimes in a Struct](#lifetimes-in-a-struct)

3. [Enums](#enums)

   - [Pattern Matching](#pattern-matching)
   - [Match Control Flow](#match-control-flow)

4. [Generics](#generics)

   - [Generic](#ge)

5. [FunctionalUtilities](#functionalutilities)
   - [into_iter()](#into_iter)
   - [collect()](#collect)
   - [filter()](#filter)
   - [binary_search_by_key](#binary_search_by_key)

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

### Lifetimes in a Struct

It helps to manage references and ensure memory safety. When it comes to structs, lifetimes can be used to define the scope of references held within the struct.

In Rust, lifetimes are used to ensure that references to data remain valid throughout their usage. A lifetime parameter is a name that represents the scope of a reference, indicating how long the reference is valid. This helps prevent dangling pointers, which occur when a reference points to data that has been freed or is no longer valid.

Think of lifetimes in Rust like expiration dates for references. Just as food isn't safe to eat after its expiration date, Rust references have lifetimes that decide how long they stay valid. Whether you explicitly state lifetimes or let the borrow checker figure it out, it's all about making sure references are used only when they should be. This keeps your program safe from memory problems and ensures it works correctly.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

---

### Enums

Enums (short for enumerations) are a way to create compound data types in Rust.

For example, we can use enums to recreate a Boolean type with two variants.i.e True and False.

```bash
enum Bool{
   True,False
}
```

// enum Bool - i.e name of the data type<br>
// True , False - i.e variant of the data type

Enums values are like structs, they can contain fields,which can be unnamed or named.In , the example below, the Alive variant contains an unnamed signed 8 bit-integer.

```bash
enum Excitement {
   Happy(i8);
   Sad,
}

let is_happy = Excitement::Happy(50);
let is_sad = Excitement::Sad;
```

### Pattern Matching

Powerful feature in Rust that allows us to destructure patterns and match values against a set of patterns.It is primarily used in `match` expressions , but also appears in other constructs like `if let` , `while let` and function parameters.

### Match Control Flow

Rust has an extremely powerful control flow construct knows as match that allows us to compare values against a series of patterns and then execute code based on which pattern matches.

```bash
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn time_to_wait(light: TrafficLight) -> u8 {
    match light {
        TrafficLight::Red => 60,
        TrafficLight::Yellow => 10,
        TrafficLight::Green => 45,
    }
}
```

**The code defines an enum TrafficLight with variants Red, Yellow, and Green. The function time_to_wait takes a TrafficLight as input and returns the corresponding time to wait: 60 seconds for Red, 10 seconds for Yellow, and 45 seconds for Green.**

### Generics

Generics are used to create special functions that can be used to represent signatures and structs.

**Generic Type**

```bash
fn largest<T>(list: &[T]) -> &T {
```

**Code 2**
This code is not generic enough to handle different types of elements in the list slice.

```bash
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

**For fixing the above code,we have to pass in Ord trait**

The Ord trait defines a method cmp that allows us to compare two elements of the same type and determine which one is greater.

```bash
fn largest<T:Ord>(list: &[T]) -> &T {
```

### FunctionalUtilities

### into_iter()
part of the 'IntoIterator' trait in Rust.It is used to convert an owned value to a vector.It is commonly used with collections (e.g arrays, vectors) to transform them into iterators.

The ownership of the original collection is transferred to the iterator.

### collect()
is an iterator method used to transform the elements of an iterator to a specific collection type.

```rust
let numbers = vec![1, 2, 3, 4, 5];
let squared_numbers: Vec<_> = numbers.into_iter().map(|x| x * x).collect();
```
### filter()
is an iterator adapter that creates a new iterator by selectively including or excluding elements based on a given condition.

It is used to perfom filtering operation on the elements of an iterator , allowing only those that satisfy the given condition

```rust
let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<_> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
```

### binary_search_by_key
performs a binary search on a sorted slice or vector , using a key extraction function to map each element to a key.

