use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::error::Error;
use std::fmt;

// Custom Error Type
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

// Function that uses Result
fn open_file(filename: &str) -> Result<File, io::Error> {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => Ok(fc),
                Err(e) => Err(e),
            },
            other_error => {
                Err(other_error.into())
            }
        },
    }
}

// Function that uses Option
fn get_optional_value(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value),
        None => None,
    }
}

// Function that propagates errors
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Function that uses the ? operator
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        // Here you can decide how to represent an io::Error as a CliError
        CliError { cause: error.to_string() }
    }
}

fn main() -> Result<(), CliError> {
    let filename = "file.txt";
    match open_file(filename) {
        Ok(file) => println!("Successfully opened file: {:?}", file),
        Err(e) => println!("Problem opening the file: {:?}", e),
    }

    let x: Option<i32> = Some(5);
    let value = get_optional_value(x);
    match value {
        Some(v) => println!("Value is {}", v),
        None => println!("Value is None"),
    }

    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Failed to read username: {}", e),
    }

    let contents = read_file(filename)?;
    println!("File contents: {}", contents);

    Err(CliError {
        cause: "Failed to parse command line arguments".to_string(),
    })
}