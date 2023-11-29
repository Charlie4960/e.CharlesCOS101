//Rust program to display food menu

use std::io;
fn main () {

    println!("Poundo Yam/Edinkaiko soup is N3,200");
    println!("Fried Rice & Chicken is N3,000");
    println!("Amala & Ewedu Soup is N2,500");
    println!("Eba & Egusi Soup is N2,500");
    println!("Eba & Egusi Soup is N2,000");
    println!("White Rice and stew is N2,500");
    
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter your order?");
    io::stdin().read_line(&mut input1).expect("An error detected");
    let _food:&str = "input1" ;

    println!("Please enter desired food quantity? ");
    io::stdin().read_line(&mut input2).expect("An error detected");
    let quantity:f32 = input2.trim().parse().expect("An error detected");

    if quantity >= 10000.00 {
        let discount = 0.05 * quantity ;
        let total_order = quantity - discount ;
        println!("Total charge is {}", total_order);
        println!("For your patronage we are giving you 5% disccount, thank you!!");
    } else {
        println!("Buy food worth 10,000 to give you a dicount next time, thank you!!");
    } 

    
}