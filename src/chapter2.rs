// Chapter 2: Ownership and Borrowing

// 1. Ownership
fn ownership() {
    // Ownership example
    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid
    // println!("{}", s1); // This will cause a compile-time error

    // Ownership with functions
    let s3 = String::from("world");
    takes_ownership(s3);
    // println!("{}", s3); // This will cause a compile-time error
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// 2. Borrowing and References
fn borrowing() {
    let s1 = String::from("hello");

    // Borrowing with immutable references
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Borrowing with mutable references
    let mut s2 = String::from("world");
    change(&mut s2);
    println!("s2 is now: {}", s2);
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

pub(crate) fn chapter2_main() {
    ownership();
    borrowing();
    lifetimes();
}