
## Chapter 6: Traits and Generics

In this chapter, we will explore Rust's trait system and how to use generics to write code that can work with different types.

1. **Traits**
2. **Trait Bounds**
3. **Generics**
4. **Associated Types**

### 1. Traits

Traits in Rust define a set of methods that a type must implement. Traits are similar to interfaces in other programming languages, but they provide more flexibility and control.

Here's an example of defining and implementing a `Summary` trait:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

In this example, the `Summary` trait defines a single method, `summarize`, which takes a reference to the `self` object and returns a `String`. The `NewsArticle` struct then implements the `Summary` trait by providing an implementation for the `summarize` method.

### 2. Trait Bounds

Trait bounds allow you to specify that a generic type parameter must implement a certain trait. This is useful when you want to constrain the types that can be used with a function or data structure.

```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

In this example, the `notify` function takes a generic parameter `T`, which must implement the `Summary` trait. This ensures that the `summarize` method can be called on the `item` parameter.

### 3. Generics

Generics allow you to write code that can work with different types. They provide a way to write code that is type-safe and reusable.

Here's an example of a generic function that finds the maximum of two values:

```rust
fn maximum<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
```

In this example, the `maximum` function takes two generic parameters `a` and `b`, which must implement the `PartialOrd` trait (a trait that provides the ability to compare values). The function then returns the larger of the two values.

### 4. Associated Types

Associated types are a way to define a type within a trait. This is useful when you want to specify a type that is associated with a particular trait implementation.

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

In this example, the `Iterator` trait defines an associated type called `Item`. This type represents the type of the elements that the iterator will return.

In the next chapter, we will explore Rust's error handling and exception management, including the use of the `Result` and `Option` types.