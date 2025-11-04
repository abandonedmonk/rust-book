#[derive(Debug)] // inspecting the state

pub enum UsState {
    Alabama,
    Alaska,
}

pub enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Matches are Exhaustive AND Catch-all
pub fn add_fancy_hat() {}
pub fn remove_fancy_hat() {}

// Matches are Exhaustive AND Catch-all
pub fn test_match() {
    let dice_roll: u8 = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        _ => remove_fancy_hat(), // catch-all pattern
    }
}
