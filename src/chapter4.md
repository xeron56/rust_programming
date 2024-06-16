## Chapter 4: Modules, Crates, and Cargo

In this chapter, we will explore Rust's module system, crates, and the Cargo build system. These concepts are essential for organizing and managing larger Rust projects.

1. **Modules**
2. **Crates**
3. **Cargo**

### 1. Modules

Rust's module system allows you to organize your code into logical units called modules. Modules can contain functions, structs, enums, and other items.

Here's an example of using modules:

```rust
// src/lib.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

pub fn run_math() {
    println!("5 + 3 = {}", math::add(5, 3));
    println!("10 - 4 = {}", math::subtract(10, 4));
}
```

In this example, we have a `math` module that contains two functions, `add` and `subtract`. The `run_math` function in the root module can then call the functions in the `math` module.

### 2. Crates

A crate is the basic unit of compilation in Rust. A crate can be a library or an executable. Crates can depend on other crates, which are called dependencies.

Here's an example of creating a new crate with Cargo:

```
$ cargo new my-crate
     Created binary (application) `my-crate` package
```

This command creates a new Rust project called `my-crate` with the following structure:

```
my-crate/
├── Cargo.toml
└── src/
    └── main.rs
```

The `Cargo.toml` file is the manifest file that contains information about the crate, including its dependencies.

### 3. Cargo

Cargo is Rust's build system and package manager. It simplifies the process of building, testing, and publishing Rust projects.

Here are some common Cargo commands:

- `cargo build`: Compiles the current project.
- `cargo run`: Builds and runs the current project.
- `cargo test`: Runs the tests for the current project.
- `cargo doc`: Builds the documentation for the current project.
- `cargo publish`: Publishes the current crate to the crates.io registry.

You can also use Cargo to add dependencies to your project. For example, to add the `rand` crate, you would add the following to your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8.4"
```

Then, you can use the `rand` crate in your code:

```rust
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {}", random_number);
}
```

In the next chapter, we will explore Rust's ownership and borrowing rules in more depth, including lifetimes and how to write safe and efficient Rust code.