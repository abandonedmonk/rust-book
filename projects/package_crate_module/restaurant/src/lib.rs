// To print the tree
// cargo modules structure
#![allow(unused_variables)]
#![allow(dead_code)]
mod front_of_house;

// Using the `use` keyword
// Submodules should NOT be announces as coder cannot fully comprehend where the submodule is coming from
// It's better to call it with the main module
// Sort of like grouping to get a better idea
// This is called `IDIOMATIC` way of using Paths

// It is basically (import) but a bit different
use crate::front_of_house::hosting;

// Makes the path a bit diff, skips `front_of_house`
// I added `as` else I am calling same hosting twice
pub use crate::front_of_house::hosting as HostingLol;

mod customer {
    // We have to define it again here, since above it is defined for the `global` scope
    // The above one is not accessible here
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waitlist(); // Absolute Path
    front_of_house::hosting::add_to_waitlist(); // Relative Path
    hosting::add_to_waitlist(); // From the `use` keyword

    let mut meal = back_of_house::Breakfast::summer("Rye"); // Order food
    meal.toast = String::from("Wheat"); // Change food
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // Not allowed to run, fruits are private
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // The elements of Struct should also be made public if we want to use it
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruits: String,
    }

    // The elements (variants) of Enums are also public if Enum itself is Public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Without these functions we cannot create instance of Breakfast
    // Because `fruits` are private, so we cannot assign it value
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("peaches"),
            }
        }
        pub fn winter(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("Grapes"),
            }
        }
    }
}
