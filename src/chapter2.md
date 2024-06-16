## Chapter 2: Ownership and Borrowing

One of the unique features of Rust is its ownership model, which is designed to ensure memory safety without the need for a garbage collector. In this chapter, we will explore the following core concepts:

1. **Ownership**
2. **Borrowing and References**
3. **Lifetimes**

### 1. Ownership

In Rust, every value has a single owner. When a value goes out of scope, the memory associated with that value is automatically deallocated. This is known as the "RAII" (Resource Acquisition Is Initialization) principle.

Here's an example of ownership:

```rust
let s = String::from("hello"); // s is the owner of the string
let t = s; // t takes ownership of the string, s is no longer valid
```

In the example above, `s` is the owner of the string `"hello"`. When `t` takes ownership of the string, `s` is no longer valid and cannot be used.

### 2. Borrowing and References

Rust allows you to borrow values using references. References provide a way to access a value without taking ownership of it. There are two types of references:

1. **Immutable references**: denoted by `&T`, where `T` is the type of the value being borrowed.
2. **Mutable references**: denoted by `&mut T`, where `T` is the type of the value being borrowed.

Here's an example of borrowing:

```rust
let s = String::from("hello");
let len = calculate_length(&s); // &s is an immutable reference
println!("The length of '{}' is {}.", s, len);
```

In the example above, `&s` is an immutable reference to the `String` value. The function `calculate_length` borrows the value without taking ownership of it.

### 3. Lifetimes

Lifetimes in Rust ensure that references are always valid. Rust's compiler uses lifetime annotations to verify that references are never used after the value they refer to has gone out of scope.

Here's an example of a function that takes two references and returns the longer of the two:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

In this example, the `'a` lifetime annotation ensures that the returned reference is valid for as long as the references passed to the function are valid.

Lifetimes are a crucial aspect of Rust's ownership and borrowing model, and understanding them is essential for writing safe and efficient Rust code.

In the next chapter, we will explore Rust's standard library and discuss some of the most commonly used data structures and collections.