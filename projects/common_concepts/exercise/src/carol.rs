// ============================ SIMPLE ====================================
// pub fn christmas_carol() {
//     let ordinal = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"];
//     for i in 0..11 {
//         println!("On the {} day of Christmas", ordinal[i]);
//         println!("My true love gave to me");
//         if i >= 11{
//             println!("Twelve drummers drumming");
//         }
//         if i >= 10 {
//             println!("Eleven pipers piping");
//         }
//         if i >= 9 {
//             println!("Ten lords a-leaping");
//         }
//         if i >= 8 {
//             println!("Nine ladies dancing");
//         }
//         if i >= 7 {
//             println!("Eight maids a-milking");
//         }
//         if i >= 6 {
//             println!("Seven swans a-swimming");
//         }
//         if i >= 5 {
//             println!("Six geese a-laying");
//         }
//         if i >= 4 {
//             println!("Five golden rings");
//         }
//         if i >= 3 {
//             println!("Four calling birds");
//         }
//         if i >= 2 {
//             println!("Three French hens");
//         }
//         if i >= 1 {
//             println!("Two turtle doves");
//             println!("And a partridge in a pear tree \n")
//         }
//         if i < 1 { 
//             println!("A partridge in a pear tree \n");
//         }
//     }
// }


// ============================= LESS SIMPLE ========================================
pub fn christmas_carol() {
    let ordinal = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", 
        "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"
    ];
    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "A partridge in a pear tree \n"
    ];

    println!("\n");
    for j in (0..=11).rev() {
        println!("On the {} day of Christmas", ordinal[11 - j]);
        println!("My true love gave to me");
        
        for k in j..12 {
            if k == 11 && j < 11 {
                println!("And a partridge in a pear tree \n");
            } else {
                println!("{}", gifts[k]);
            }
        }
    }
}