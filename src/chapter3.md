
## Chapter 3: Standard Library and Data Structures

In this chapter, we will explore some of the most commonly used data structures and collections provided by Rust's standard library. We will cover the following topics:

1. **Vectors**
2. **Strings**
3. **Hash Maps**
4. **Options and Results**

### 1. Vectors

Vectors are growable arrays that can hold values of the same type. They are stored on the heap, unlike arrays which are stored on the stack.

Here's an example of working with vectors:

```rust
// Creating a vector
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);

// Accessing elements
println!("The first element is: {}", v[0]);
println!("The last element is: {}", v[v.len() - 1]);

// Iterating over a vector
for i in &v {
    println!("{}", i);
}
```

### 2. Strings

Rust has two string types: `String` and `&str`. `String` is a growable, heap-allocated string type, while `&str` is an immutable reference to a string slice.

```rust
// Creating a String
let mut s = String::from("hello");
s.push_str(", world!");
println!("{}", s); // Output: "hello, world!"

// Slicing a String
let hello = &s[0..5];
println!("{}", hello); // Output: "hello"
```

### 3. Hash Maps

Hash maps are a data structure that store key-value pairs. They are useful for lookup, insertion, and deletion operations.

```rust
use std::collections::HashMap;

// Creating a Hash Map
let mut scores = HashMap::new();
scores.insert("Blue", 10);
scores.insert("Yellow", 50);

// Accessing values
println!("The score for Blue is: {}", scores["Blue"]);

// Iterating over a Hash Map
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### 4. Options and Results

Rust's standard library provides two enums, `Option<T>` and `Result<T, E>`, to handle the possibility of a value being absent or an operation failing.

```rust
// Option<T>
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// Result<T, E>
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

let result = divide(10, 2);
println!("Result: {:?}", result); // Output: Result: Ok(5)
```

In the next chapter, we will explore Rust's module system, crates, and the cargo build system, which are essential for organizing and managing larger Rust projects.