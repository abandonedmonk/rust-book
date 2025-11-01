#![allow(warnings)] // To silence VAR not used warnings
mod rectangles;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // Field Init Shorthand Syntax
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// ============ TUPLE STRUCT =============
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ============ UNIT STRUCT ============
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("abc@gmail.com"),
        username: String::from("abc"),
        sign_in_count: 1,
    };

    // Struct Update Syntax
    let user2 = User {
        email: String::from("mewabc@gmail.com"),
        ..user1 // Rest of the values are copied from User 1
    };

    user1.email = String::from("ehe@gmail.com");

    // println!("Username: {}", user1.username); // Cannot be used anymore since `user1.username` was borrowed by `user2`
    println!("Email: {}", user1.email); // `user1.email` can be used since it was not borrowed
    println!("Active: {}", user1.active); // `user1.active | sign_in_count` can also be used since they use the COPY trait

    // ======================== TUPLE STRUCT =========================
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // ======================== UNIT STRUCT =========================
    let subject = AlwaysEqual;

    // ######################### SUB PROJECT #############################
    let rect1 = rectangles::Rectangle {
        width: dbg!(30),
        height: 50,
    };
    let rect2 = rectangles::Rectangle {
        width: dbg!(40),
        height: 20,
    };

    println!("The are of the rectangle is {}", rect1.area());
    // println!("Rect1 is {rect1:#?}");
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
