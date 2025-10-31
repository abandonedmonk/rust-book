fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    scoping();
}

fn scoping() {
    let x = 6;
    let spaces = "    ";
    let _spaces = spaces.len();
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }

    println!("The value of x in outer scope is: {x}");
}