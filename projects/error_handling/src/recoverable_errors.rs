// Recoverable Errors with Result<T, E>
// Result enum: Ok(T) for success, Err(E) for failure

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

#[allow(unused)]
pub fn recoverable_errors_func() {
    println!("\n=== Recoverable Errors with Result ===\n");

    // File::open returns Result<File, Error>
    let greeting_file_res = File::open("hello.txt");

    // Pattern 1: Matching on Result and handling specific errors
    let greeting_file = match greeting_file_res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // If file not found, create it
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            // For other errors, panic
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // Pattern 2: unwrap() - panics if Err, returns value if Ok
    // Use for prototyping, not production
    // let greeting_file = File::open("lmao.txt").unwrap();

    // Pattern 3: expect() - like unwrap but with custom error message
    // Better for production: provides context for debugging
    let greeting_file =
        File::open("lmao.txt").expect("hello.txt should be included in this project");
}

// ============ Propagating Errors ============

// Pattern 1: Verbose error propagation with match
#[allow(dead_code)]
pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // If opening fails, return the error to caller
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // If reading fails, return the error to caller
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Pattern 2: Using ? operator (shortcut for error propagation)
// ? will return early if Err, unwrap if Ok
#[allow(dead_code)]
pub fn read_username_from_file_withq() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // Returns Err if fails
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // Returns Err if fails
    Ok(username)
}

// Pattern 3: Chaining ? operator (even more concise)
#[allow(dead_code)]
pub fn read_username_from_file_withq_oneline() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Pattern 4: Using built-in functions (most concise)
// fs::read_to_string does all the work for us
#[allow(dead_code)]
pub fn read_username_from_file_builtin() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Using ? with Option<T>
// ? works with Option too: returns None if None, unwraps if Some
#[allow(dead_code)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // If no lines: returns None
    // If no chars: returns None
    // Otherwise: returns Some(char)
}
