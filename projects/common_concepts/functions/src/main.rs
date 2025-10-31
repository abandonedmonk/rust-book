fn main() {
    println!("Hello, world!");

    another_function();

    print_labeled_measurement(5, 'c');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    println!("The value of five() is: {}", five());
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value:i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}