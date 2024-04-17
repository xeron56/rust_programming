// Chapter 5: Ownership and Borrowing Revisited

// 1. Ownership Rules
fn ownership_rules() {
    // Rule 1: Each value in Rust has a variable that's called its owner.
    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid

    // Rule 2: There can only be one owner at a time.
    // let s3 = s1; // This will cause a compile-time error

    // Rule 3: When the owner goes out of scope, the value will be dropped.
    // Implicit drop when s2 goes out of scope
}

// 2. Borrowing and References
fn borrowing_and_references() {
    let s1 = String::from("hello");

    // Immutable reference
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable reference
    let mut s2 = String::from("world");
    change(&mut s2);
    println!("s2 is now: {}", s2);

    // Multiple immutable references
    let r1 = &s2;
    let r2 = &s2;
    println!("{} and {}", r1, r2);

    // Mutable and immutable references cannot coexist
    // let r3 = &mut s2; // This will cause a compile-time error
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", changed!");
}

// 3. Lifetimes
fn lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 4. Common Ownership Patterns
fn ownership_patterns() {
    // Passing ownership to a function
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // This will cause a compile-time error

    // Returning ownership from a function
    let s = gives_ownership();
    println!("{}", s);

    // Storing references in data structures
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("The first element is: {}", first);
    // v.push(4); // This will cause a compile-time error

    // Dealing with optional data
    let x: Option<&str> = None;
    if let Some(value) = x {
        println!("{}", value);
    }

    // Handling errors with Result<T, E>
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub(crate) fn chapter5_main() {
    ownership_rules();
    borrowing_and_references();
    lifetimes();
    ownership_patterns();
}