// Error Handling in Rust - Chapter 9
// Two types of errors:
// 1. Unrecoverable errors: panic! macro (program stops)
// 2. Recoverable errors: Result<T, E> (can handle and continue)

mod panic_ex;
mod recoverable_errors;

fn main() {
    // Unrecoverable errors - program will crash
    // panic_ex::panic_func();

    // Recoverable errors - we can handle them gracefully
    recoverable_errors::recoverable_errors_func();
}

// Alternative: Using Result in main
// main can return Result to propagate errors to the runtime
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }
