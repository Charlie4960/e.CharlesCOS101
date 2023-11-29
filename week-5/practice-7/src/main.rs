// the use of while loop

use std::io;

fn main() {

    println!("Enter a number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read");
    let mut num:i32 = input1.trim().parse().expect("Failed to read");

    while num < 10 {

        println!("Inside lop number value is {}", num);
        num = num + 1;
    }
    println!("outside loop number value is {}", num);
}