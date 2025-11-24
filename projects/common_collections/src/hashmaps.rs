use std::collections::HashMap;

pub fn hashmaps_func() {
    println!("\n=== Creating HashMaps ===\n");

    // Create a new empty HashMap
    let mut scores = HashMap::new();

    // Insert key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Initial scores: {:?}\n", scores);

    println!("=== Accessing Values ===\n");

    // Method 1: get() - returns Option<&V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for {}: {}", team_name, score);

    // Non-existent key returns None
    let score = scores.get("Red").copied().unwrap_or(0);
    println!("Score for Red (default): {}\n", score);

    // Iterating over key-value pairs
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    println!("\n=== Ownership ===\n");

    // Types that implement Copy (like i32) are copied
    // Owned values (like String) are moved into the HashMap
    let field_name = String::from("Favorite color");
    let field_val = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_val);
    // println!("{}", field_name); // Error: field_name was moved
    println!("Map owns the inserted values: {:?}\n", map);

    println!("=== Updating Values ===\n");

    // Strategy 1: Overwriting a value
    scores.insert(String::from("Green"), 10);
    println!("After inserting Green=10: {:?}", scores);
    scores.insert(String::from("Green"), 25);
    println!("After overwriting Green=25: {:?}\n", scores);

    // Strategy 2: Insert only if key doesn't exist
    println!("Using entry().or_insert():");
    scores.entry(String::from("Yellow")).or_insert(30); // Exists, no change
    scores.entry(String::from("Red")).or_insert(30); // New, inserted
    println!("After conditional inserts: {:?}\n", scores);

    // Strategy 3: Update based on old value
    println!("=== Word Counter Example ===\n");
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1; // Dereference and increment
    }

    println!("Text: '{}'", text);
    println!("Word frequencies: {:?}", word_count);
}
