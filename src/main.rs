mod chapter1;
mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;

use std::io;

fn main() {
    println!("Enter the chapter number to run the code");
    let mut chapter = String::new();
    io::stdin().read_line(&mut chapter).expect("Failed to read line");
    let chapter: u32 = chapter.trim().parse().expect("Please type a number!");

    match chapter {
        1 => chapter1::chapter1_main(),
        2 => chapter2::chapter2_main(),
        3 => chapter3::chapter3_main(),
        //4 => chapter4::chapter4_main(),
        5 => chapter5::chapter5_main(),
        6 => chapter6::chapter6_main(),
        _ => println!("Chapter not found"),
    }
}
       
