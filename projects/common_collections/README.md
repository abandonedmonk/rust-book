# Common Collections - Learning Exercises

This project contains learning examples for Rust's common collections: Vectors, Strings, and HashMaps.

## TODO: Practice Exercises

### 1. Statistics Calculator
Given a list of integers, use a vector and return:
- **Median**: When sorted, the value in the middle position
- **Mode**: The value that occurs most often (use a HashMap to track frequencies)

**Example:**
```rust
let numbers = vec![1, 2, 2, 3, 3, 3, 4, 5];
// Median: 3
// Mode: 3 (appears 3 times)
```

---

### 2. Pig Latin Converter
Convert strings to pig latin with the following rules:
- **Consonant start**: Move the first consonant to the end and add "ay"
  - `"first"` → `"irst-fay"`
- **Vowel start**: Add "hay" to the end
  - `"apple"` → `"apple-hay"`

**Important**: Keep in mind UTF-8 encoding details when working with characters!

**Example:**
```rust
pig_latin("first");  // "irst-fay"
pig_latin("apple");  // "apple-hay"
pig_latin("hello");  // "ello-hay"
```

---

### 3. Employee Department Manager
Using a HashMap and vectors, create a text interface to manage employees in departments:

**Features:**
- Add employees to departments
  - `"Add Sally to Engineering"`
  - `"Add Amir to Sales"`
- Retrieve all people in a specific department (sorted alphabetically)
- Retrieve all people in the company grouped by department (sorted alphabetically)

**Example:**
```rust
// Commands:
// "Add Sally to Engineering"
// "Add Amir to Sales"
// "Add Bob to Engineering"
// "List Engineering"  → ["Bob", "Sally"]
// "List All"          → Engineering: ["Bob", "Sally"], Sales: ["Amir"]
```

---

## Current Learning Modules

- **vectors.rs**: Vector creation, accessing, and heterogeneous storage with enums
- **strings.rs**: String creation, updating, concatenation, UTF-8 encoding, and iteration
- **hashmaps.rs**: HashMap creation, accessing, ownership, and update strategies
