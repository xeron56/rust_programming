// Chapter 1: Introduction to Rust

// 1. Variable Bindings and Mutability
fn variables_and_mutability() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is now: {}", y);

    // Shadowing
    let x = x + 1;
    println!("The value of x is now: {}", x);

    let x = x * 2;
    println!("The value of x is now: {}", x);
}

// 2. Data Types
fn data_types() {
    // Integers
    let a: i32 = 42;
    let b: u8 = 255;

    // Floating-point numbers
    let x: f64 = 3.14;
    let y: f32 = 2.71828;

    // Booleans
    let is_true: bool = true;
    let is_false: bool = false;

    // Characters
    let c: char = 'A';
    let emoji: char = '😀';

    // Tuples
    let tuple: (i32, f64, u8) = (42, 3.14, 255);
    println!("Tuple: {:?}", tuple);

    // Arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
}

// 3. Control Flow
fn control_flow() {
    // If-else statements
    let x = 5;
    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }

    // Loops
    // For loop
    for i in 0..5 {
        println!("{}", i);
    }

    // While loop
    let mut y = 0;
    while y < 5 {
        println!("{}", y);
        y += 1;
    }

    // Infinite loop
    // loop {
    //     println!("This will run forever!");
    // }

    // Match statement
    let num = 4;
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 | 4 | 5 => println!("Three, four, or five"),
        _ => println!("Something else"),
    }
}

pub(crate) fn chapter1_main() {
    variables_and_mutability();
    data_types();
    control_flow();
}