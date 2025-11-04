#![allow(dead_code)]
#![allow(unused_variables)]

use crate::match_control::value_in_cents;
mod match_control;

// IP Address Enum
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
// IP Address Enum used as a type in a Function
fn route(ip_kind: IpAddrKind) {}

// Enum Message
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Applying some method on Message
impl Message {
    fn call(&self) {}
}

fn main() {
    // Calling the IP Enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Putting a value in the IP Enum
    route(IpAddrKind::V6(String::from("127.0.0.1")));
    let m = Message::Write(String::from("Hello"));
    m.call();

    // Using Option Enum (inbuilt)
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // Patterns that bind to Values
    let coin = value_in_cents(match_control::Coin::Quarter(
        match_control::UsState::Alabama,
    ));
    println!("Coin: {}", coin);
}
