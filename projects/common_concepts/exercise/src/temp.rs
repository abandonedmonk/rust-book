use std::io;

// fn main() {
//      run_fac();
// }

fn input_for_fac() -> (char, f64) {
    // Get temperature category from user (Celsius or Fahrenheit)
    let mut cat = String::new();
    println!("Enter Celsius or Farenheit [c/f]: ");
    io::stdin().read_line(&mut cat).unwrap();
    let cat = cat.chars().next().unwrap();

    // Get temperature value from user
    let mut num = String::new();
    println!("Enter the temp: ");
    io::stdin().read_line(&mut num).expect("Failed to read line.");
    let num: f64 = num.trim().parse().expect("Please enter a valid number.");

    return (cat, num);
}

// Temperature conversion function
// Converts between Celsius and Fahrenheit based on category
fn fahrenheit_and_celsius(temperature: f64, category: char) -> f64 {
    if category == 'c' {
        // Convert Celsius to Fahrenheit: (C × 9/5) + 32
        let temp = (temperature * (9.0 / 5.0)) + 32.0;
        return temp;
    } else if category == 'f' {
        // Convert Fahrenheit to Celsius: (F - 32) × 5/9
        let temp = (temperature - 32.0) * (5.0 / 9.0);
        return temp;
    } else {
        // Invalid category
        return -1.0;
    }
}

pub fn run_fac() {
    // Get input from user
    let (cat, num) = input_for_fac();
    
    // Convert temperature and display result
    let temp = fahrenheit_and_celsius(num, cat);
    if temp == -1.0 {
        println!("Wrong temperature category");
        return;
    }
    if cat == 'c' {
        println!("The temperature changed from {num}°C to {temp:.1}°F")
    } else {
        println!("The temperature changed from {num}°F to {temp:.1}°C")
    }  
}