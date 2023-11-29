// Algorithm

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();

    println!("Enter name: ");
    io::stdin().read_line(&mut input1).expect("Not  valid");
    let name = input1.trim();

    println!("Enter Date of birth(age): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:u32 = input2.trim().parse().expect("Not a valid string");

    println!("Enter email address: ");
    io::stdin().read_line(&mut input3).expect("Not valid");
    let email =  input3.trim();

    println!("Enter phone number: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let phno:f64 = input4.trim().parse().expect("Not a valid string");

    println!("Enter number of siblings: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let nos:u32 = input5.trim().parse().expect("Not a valid string");

    println!("Enter number of children: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let noc:u32 = input6.trim().parse().expect("Not a valid string");

    println!("Enter medical diagnosis: ");
    io::stdin().read_line(&mut input7).expect("Not valid");
    let md =  input7.trim();

    println!("Enter village of residence: ");
    io::stdin().read_line(&mut input8).expect("Not  valid");
    let vof = input8.trim();

    if md == "Alhzhemier" && age > 50 && noc > 4 && vof == "Akpabiom" {
        println!(" {} {} {} {} {} {} {} {}", name, age, email, phno, nos, noc, md, vof);
        let charge1:f64 = 1200000.0 ;
        let discount1 = 0.2 * charge1;  
        let totalcharge1 = charge1 - discount1;
        println!("Total charge is {}", totalcharge1);
    } else {
        println!(" Normal charge is 1200000");
    }
    
    if md == "Arrythmia" && age > 30 && nos > 4 && vof == "Ngbauji" {
        println!(" {} {} {} {} {} {} {} {}", name, age, email, phno, nos, noc, md, vof);
        let charge2:f64 = 550000.0 ;
        let discount2 =  0.05 * charge2 ;
        let totalcharge2 = charge2 - discount2;
        println!("Total charge is {}", totalcharge2)
    } else {
        println!(" Normal charge is 550000");
    }
    

    if md == "Chronic-Kidney-Disease" && age > 40 && nos > 3 && noc > 3
     && vof == "Atabrikang" {
        println!(" {} {} {} {} {} {} {} {}", name, age, email, phno, nos, noc, md, vof);
        let charge3:f64 = 1500000.0;
        let discount3 = 0.15 * charge3 ;
        let totalcharge3 = charge3 - discount3;
        println!("Total charge is {}", totalcharge3);
     } else {
        println!("Normal charge is 1500000")
     }
     
     
    if md == "Diabetes" && age > 28 && age < 45 && noc >2 && nos< 4 && vof == "Okorobilom" {
        println!(" {} {} {} {} {} {} {} {}", name, age, email, phno, nos, noc, md, vof);
        let charge4:f64 = 800000.0;
        let discount4 = 0.1 * charge4;
        let totalcharge4 = charge4 - discount4;
        println!("Total charge is {}", totalcharge4);
    } else {
        println!(" Normal charge is 800000");
    }

    if md == "Arthritis" && age > 58 && noc > 5 && nos > 5 && vof == "Emeremen" {
        println!(" {} {} {} {} {} {} {} {}", name, age, email, phno, nos, noc, md, vof);
        let charge5:f64 = 450000.0;
        let discount5 = 0.1 * charge5;
        let totalcharge5 = charge5 - discount5;
    } else {
        println!("Normal charge is 450000 ");
    }
}









    
