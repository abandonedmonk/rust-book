#[allow(unused)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn timepass_vec() {
    println!("=== Vector Basics ===\n");

    // Creating vectors: two ways
    let mut v: Vec<i32> = Vec::new(); // Empty vector, type must be specified
    let vec_with_val = vec![1, 2, 3]; // vec! macro, type inferred from values
    println!("Initial vector: {:?}", vec_with_val);

    // Adding elements with push()
    v.push(5);
    v.push(6);
    v.push(8);
    v.push(9);
    println!("After pushing: {:?}\n", v);

    // Method 1: Indexing with [] - panics if index is out of bounds
    let third: &i32 = &v[2]; // Returns reference to element
    println!("Accessing v[2]: {}", third);

    // Method 2: get() - returns Option, safe for invalid indices
    let third: Option<&i32> = v.get(2);
    match third {
        Some(val) => println!("Using get(2): {}", val),
        None => println!("No element at index 2"),
    }

    // Invalid index example
    match v.get(100) {
        Some(val) => println!("Element at 100: {}", val),
        None => println!("get(100) safely returned None\n"),
    }

    // Storing different types using enums
    println!("=== Heterogeneous Vector ===");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Spreadsheet row with mixed types: {} elements", row.len());
}
