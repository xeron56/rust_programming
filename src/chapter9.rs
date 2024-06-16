// Chapter 9: Advanced Features

// 1. Macros

// Simple println_plus macro
macro_rules! println_plus {
    ($($arg:tt)*) => {
        println!("{} = {:?}", stringify!($($arg)*), $($arg)*);
    };
}

fn macro_example_1() {
    let x = 42;
    println_plus!(x);
    println_plus!(x + 2);
    println_plus!(String::from("hello"));
}

// Macro with pattern matching
macro_rules! define_function {
    ($func_name:ident() $body:block) => {
        fn $func_name() $body
    };
}

define_function!(hello_world() {
    println!("Hello, world!");
});

fn macro_example_2() {
    hello_world();
}

// Recursive macros
macro_rules! factorial {
    (0) => (1);
    ($n:expr) => ($n * factorial!($n - 1));
}

fn macro_example_3() {
    println!("Factorial of 5 is {}", factorial!(5));
}

// Declarative macros
#[macro_export]
macro_rules! vec_macro {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn macro_example_4() {
    let v = vec_macro!(1, 2, 3, 4, 5);
    println!("{:?}", v);
}

// Procedural macros
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyMacro)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_my_macro(&ast)
}

fn impl_my_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let expanded = quote! {
        impl MyMacro for #name {
            fn my_method(&self) {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };
    expanded.into()
}

trait MyMacro {
    fn my_method(&self);
}

fn macro_example_5() {
    #[derive(MyMacro)]
    struct MyStruct;

    let my_struct = MyStruct {};
    my_struct.my_method();
}

fn main() {
    macro_example_1();
    macro_example_2();
    macro_example_3();
    macro_example_4();
    macro_example_5();
}