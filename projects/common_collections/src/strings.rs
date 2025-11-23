#[allow(dead_code)]
#[allow(unused)]
pub fn string_func() {
    println!("\n=== Creating Strings ===\n");

    // Method 1: String::new() - creates empty string
    let mut s = String::new();
    println!("Empty string: '{}'", s);

    // Method 2: String::from() - creates from string literal
    let s = String::from("initial contents");
    println!("String::from(): '{}'", s);

    // UTF-8 support - any valid UTF-8 can be stored
    let hello = String::from("नमस्ते");
    println!("UTF-8 string: '{}'", hello);

    // Method 3: .to_string() - converts &str to String
    let data = "initial contents";
    let s = data.to_string();
    println!("Using .to_string(): '{}'\n", s);

    println!("=== Updating Strings ===\n");

    // push_str() - appends a string slice (doesn't take ownership)
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("After push_str('{}'), s = '{}'", s2, s);
    println!("s2 still valid: '{}'", s2); // s2 not moved

    // push() - appends a single character
    s.push('!');
    println!("After push('!'), s = '{}'\n", s);

    println!("=== Concatenation ===\n");

    // Method 1: + operator (moves first string, borrows second)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 moved, s2 borrowed
    println!("Using +: '{}'", s3);
    // println!("{}", s1); // Error: s1 was moved

    // Method 2: format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Using format!: '{}'", s);
    println!("All variables still valid: {}, {}, {}\n", s1, s2, s3);

    println!("=== String Indexing & Internal Representation ===\n");

    // Rust strings can't be indexed by position (no s[0])
    // Reason: UTF-8 encoding complexity
    let hello = String::from("नमस्ते");
    println!("String: '{}'", hello);
    println!("Bytes: {} bytes", hello.len()); // 18 bytes, not 6!
    println!("Chars: {} chars", hello.chars().count()); // 6 scalar values
    // Grapheme clusters: 4 ("न", "म", "स्", "ते")

    println!("\n=== String Slicing ===\n");

    // Slicing by byte index (use with caution!)
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Each Cyrillic char is 2 bytes
    println!("Original: '{}'", hello);
    println!("Slice [0..4]: '{}' (first 2 chars)\n", s);
    // Note: &hello[0..1] would panic - cuts mid-character!

    println!("=== Iterating Over Strings ===\n");

    let text = "Зд";
    println!("Text: '{}'", text);

    // Iterate over Unicode scalar values (chars)
    print!("Chars: ");
    for c in text.chars() {
        print!("'{}' ", c);
    }
    println!();

    // Iterate over raw bytes
    print!("Bytes: ");
    for b in text.bytes() {
        print!("{} ", b);
    }
    println!();
}
