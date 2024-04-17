// Chapter 3: Standard Library and Data Structures

// 1. Vectors
fn vectors() {
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

    // Removing elements
    let last_element = v.pop();
    println!("Removed the last element: {:?}", last_element);

    // Using the `vec!` macro
    let v2 = vec![4, 5, 6];
    for i in &v2 {
        println!("{}", i);
    }
}

// 2. Strings
fn strings() {
    // Creating a String
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s); // Output: "hello, world!"

    // Slicing a String
    let hello = &s[0..5];
    println!("{}", hello); // Output: "hello"

    // Concatenating Strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3); // Output: "Hello, world!"

    // Using the `format!` macro
    let s4 = format!("{} {}", "Rust", "is awesome!");
    println!("{}", s4); // Output: "Rust is awesome!"
}

// 3. Hash Maps
fn hash_maps() {
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

    // Updating values
    scores.insert("Blue", 25);
    println!("The new score for Blue is: {}", scores["Blue"]);

    // Removing key-value pairs
    scores.remove("Yellow");
}

// 4. Options and Results
fn options_and_results() {
    // Option<T>
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    // Unwrapping an Option
    if let Some(value) = x {
        println!("x is {}", value);
    }

    // Result<T, E>
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

pub(crate) fn chapter3_main() {
    vectors();
    strings();
    hash_maps();
    options_and_results();
}