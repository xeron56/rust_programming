# Rust Programming: A Comprehensive Tutorial

## Chapter 8: Concurrency

In this chapter, we will explore Rust's concurrency features, including threads, message passing, and shared-state concurrency.

1. **Threads**
2. **Message Passing**
3. **Shared-State Concurrency**
4. **Concurrency Primitives**

### 1. Threads

Rust's standard library provides a thread module that allows you to create and manage threads. Here's an example of creating and joining threads:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from the new thread!");
    });

    handle.join().unwrap();
    println!("Hello from the main thread!");
}
```

In this example, we create a new thread using the `thread::spawn` function, which takes a closure as an argument. We then wait for the new thread to finish by calling `join` on the thread handle.

### 2. Message Passing

Rust also supports message passing concurrency using the `std::sync::mpsc` (Mutex, Sender, Receiver) module. Here's an example of using channels to communicate between threads:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

In this example, we create a channel using `mpsc::channel()`, which returns a `Sender` and a `Receiver`. We then spawn a new thread and send a message through the `Sender`. Finally, we receive the message from the `Receiver` in the main thread.

### 3. Shared-State Concurrency

Rust also provides tools for shared-state concurrency, such as `std::sync::Mutex` and `std::sync::Arc`. Here's an example of using a `Mutex` to protect shared data:

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

In this example, we use an `Arc` (Atomic Reference Counted) to share a `Mutex` between multiple threads. Each thread increments the shared counter, and we wait for all threads to finish before printing the final result.

### 4. Concurrency Primitives

Rust also provides a number of concurrency primitives, including:

- `std::sync::Barrier`: Allows a set of threads to synchronize at a particular point.
- `std::sync::Condvar`: Allows threads to wait for a condition to become true.
- `std::sync::RwLock`: Provides a read-write lock, allowing multiple readers or a single writer.
- `std::sync::Semaphore`: Allows you to control the number of concurrent access to a resource.

These primitives can be used to build more complex concurrent systems in Rust.

In the next chapter, we will explore Rust's advanced features, including macros, unsafe code, and FFI (Foreign Function Interface).