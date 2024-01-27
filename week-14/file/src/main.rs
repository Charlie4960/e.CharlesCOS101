use std::io::Read;
use std::io;

fn main(){
    println!("This is globa.com data_base
    Click 1. For Administrator
    Click 2. For project manager
    Click 3. For Employee
    Click 4. For Customer
    Click 5. For Vendor
    Click 6. To quit");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice:u32= input.trim().parse().expect("Failed to read line");

    if choice == 1 {
        choice1();
    } else if choice == 2 {
        choice2();
    } else if choice == 3 {
         choice3();
    } else if choice == 4 {
        choice4();
    } else if choice == 5 {
        choice5();
    } else {
        println!("Not among options");
    }
    
}
fn choice1() {
    let mut file = std::fs::File::open("globacom_db.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn choice2() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn choice3() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn choice4() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
fn choice5() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}