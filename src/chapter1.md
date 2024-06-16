

## Chapter 1: Introduction to Rust

Rust is a systems programming language that emphasizes performance, safety, and concurrency. It was created by Mozilla Research and has gained significant popularity in recent years, particularly in the areas of web development, game development, and system programming.

In this chapter, we will cover the following core features of Rust:

1. **Variable Bindings and Mutability**
2. **Data Types**
3. **Control Flow**

### 1. Variable Bindings and Mutability

In Rust, variables are declared using the `let` keyword. By default, variables in Rust are immutable, meaning their values cannot be changed after they are assigned. To create a mutable variable, you need to use the `mut` keyword:

```rust
// Immutable variable
let x = 5;

// Mutable variable
let mut y = 10;
y = 15; // This is valid
```

Rust also supports shadowing, which allows you to redefine a variable with the same name:

```rust
let x = 5;
let x = x + 1; // x is now 6
let x = x * 2; // x is now 12
```

### 2. Data Types

Rust has a wide range of built-in data types, including:

- **Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- **Floating-point numbers**: `f32`, `f64`
- **Booleans**: `bool`
- **Characters**: `char`
- **Tuples**: `(i32, f64, u8)`
- **Arrays**: `[i32; 5]`

Rust also supports user-defined data types, such as structs and enums, which we will cover in later chapters.

### 3. Control Flow

Rust provides several control flow constructs, including:

- **If-else statements**:
  ```rust
  if x > 0 {
      println!("x is positive");
  } else if x < 0 {
      println!("x is negative");
  } else {
      println!("x is zero");
  }
  ```
- **Loops**:
  - `for` loop:
    ```rust
    for i in 0..5 {
        println!("{}", i);
    }
    ```
  - `while` loop:
    ```rust
    let mut x = 0;
    while x < 5 {
        println!("{}", x);
        x += 1;
    }
    ```
  - `loop` (infinite loop):
    ```rust
    loop {
        println!("This will run forever!");
    }
    ```
- **Match statements**:
  ```rust
  let x = 5;
  match x {
      1 => println!("One"),
      2 => println!("Two"),
      3 | 4 | 5 => println!("Three, four, or five"),
      _ => println!("Something else"),
  }
  ```

In the next chapter, we will dive deeper into Rust's ownership model and borrowing rules, which are essential for understanding Rust's safety guarantees.