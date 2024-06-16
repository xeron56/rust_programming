
## Chapter 7: Error Handling

In this chapter, we will explore Rust's error handling mechanisms, including the use of the `Result` and `Option` types.

1. **Handling Errors with `Result`**
2. **Handling Null Values with `Option`**
3. **Propagating Errors**
4. **Custom Error Types**
5. **The `?` Operator**

### 1. Handling Errors with `Result`

Rust encourages the use of the `Result<T, E>` type to handle errors. `Result` is an enum with two variants: `Ok(T)` and `Err(E)`, where `T` is the success value and `E` is the error value.

Here's an example of using `Result`:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = File::open("file.txt");
    let _file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

In this example, we use a `match` expression to handle the possible `Result` values returned by `File::open`.

### 2. Handling Null Values with `Option`

The `Option<T>` type is used to handle the possibility of a value being absent. It has two variants: `Some(T)` and `None`.

```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    if let Some(value) = x {
        println!("x is {}", value);
    }

    if let Some(value) = y {
        println!("y is {}", value);
    } else {
        println!("y is None");
    }
}
```

In this example, we use `if let` to unwrap the values of the `Option` types.

### 3. Propagating Errors

Rust provides the `?` operator to simplify error propagation. This operator can be used to propagate errors from one function to another.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

In this example, the `?` operator is used to propagate any errors that occur while opening the file or reading its contents.

### 4. Custom Error Types

You can define your own error types to provide more context and information about the errors that can occur in your application.

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CliError {
    cause: String,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLI Error: {}", self.cause)
    }
}

impl Error for CliError {}

fn main() -> Result<(), CliError> {
    Err(CliError {
        cause: "Failed to parse command line arguments".to_string(),
    })
}
```

In this example, we define a custom `CliError` type that implements the `Error` trait, allowing it to be used in the same way as other error types.

### 5. The `?` Operator

The `?` operator is a shorthand for propagating errors. It can be used to simplify error handling and make your code more concise.

```rust
use std::fs::File;
use std::io::Read;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

In this example, the `?` operator is used to propagate any errors that occur while opening the file or reading its contents.

In the next chapter, we will explore Rust's concurrency features, including threads, message passing, and shared-state concurrency.