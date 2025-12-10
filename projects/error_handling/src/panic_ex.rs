// Unrecoverable Errors with panic!
// When panic! executes, the program:
// 1. Prints a failure message
// 2. Unwinds and cleans up the stack
// 3. Quits

#[allow(unused)]
pub fn panic_func() {
    // Method 1: Explicit panic with custom message
    // panic!("crash and burn");

    // Method 2: Panic from invalid operation
    // Accessing invalid index causes panic
    let v = vec![1, 2, 3];
    v[99]; // Panics: index out of bounds

    // To see backtrace: set RUST_BACKTRACE=1 (Windows CMD)
    // Or: $env:RUST_BACKTRACE=1 (PowerShell)
}
