#![allow(unused)]
// When bringing in structs, enums use `Idiomatic` way
use std::collections::HashMap;

// Also needed when 2 Submodules have same name
use std::fmt;
use std::io;

// Using `as` keyword to give new name (also in Python)
use std::io::Result as IoResult;

// Using external packages
use rand::Rng;

// Grouping (in Python we add comma)
use std::{self, i16};
use std::{cmp::Ordering, i128};

// Calling all public functions within a module
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}

// Both have Result, so directly calling them above would have caused issues
fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
