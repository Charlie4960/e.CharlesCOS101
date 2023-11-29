//Rust program to read the height of a person
// and then print if person is tall, dwarf
// or average height person

use std::io;

fn main() {
    let mut input1 = String::new();
    

  println!("\nEnter height (in centimeters): ");
  io::stdin().read_line(&mut input1).expect("Not a valid height");
 let height:f32 = input1.trim().parse().expect("Not a valid height");

  if height >= 150.00 && height <= 170.00 {
    println!("You are of average height person");
  } else if height > 170.00 && height <= 195.0 {
    println!("You are tall");
  } else if height < 150.00 && height > 100.0 {
    println!("You are a dwarf!");
  } else {
    println!("Height not found");
  }

}