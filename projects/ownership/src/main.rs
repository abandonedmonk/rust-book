#![allow(warnings)] // To silence VAR not used warnings

mod ref_and_borr;

fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{s}");

    // =========================== MOVE ============================
    let mut s2 = s;
    // println!("{s}"); // No longer valid
    println!("S2: {s2}");



    // Had to  `mut` it above to change it below
    // Even while using s2 we have to mut it
    s2 = String::from("ahoy");
    println!("S2: {s2}");



    // ========================= DEEP COPY ==========================
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");



    // ========================= FUNCS and ONSHP ===========================
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.



    // ========================== RETURN VALS and SCOPE ============================
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3



    // =========================== RETURN ONSHP =============================
    let onshp = String::from("hello");
    let (s2, len) = calculate_string(onshp);
    println!("The length of '{s2}' is {len}.");


    // ====================== BORROWING ===========================
    let sborr = String::from("Hello");
    let len = ref_and_borr::calculate_length(&sborr);
    println!("The length of '{}' with borrowing is {}", sborr, len);


    // ===================== MUTABLE Ref ============================
    let mut ref_s = String::from("Yokoso");
    ref_and_borr::change(&mut s);
}



// ======================== FUNCTIONS ===================================


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("Takes ONSHP: {some_string}");
} // Here, some_string goes out of scope and `drop` is called. 
  // The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("Makes copy: {some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {       // `gives_ownership` will move its
                                       // return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    // some_string is returned and moves out to the calling function
    some_string                        
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_string(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}