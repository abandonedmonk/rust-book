// The inner elements also have to be public in order to be used
// Separated it from lib.rs
// Prev in lib.rs with name `front_of_house`
pub mod hosting;
mod serving {
    fn take_order() {}
    fn serve_order() {
        super::hosting::add_to_waitlist(); // Using super to access sibling module
    }
    fn take_payment() {}
}
