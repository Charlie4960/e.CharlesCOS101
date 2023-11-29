fn main() {
    let dk1:f64 = 80.0;
    let t1:f64 = 2.0;
    let c:f64 = 1.609;
    let dm1 = dk1 * c;

    //speed of the journey
    let s1 = dm1 / t1;
    println!("The speed is:{}", s1);
    
    //Another Question
    let dk2:f64 = 120.0;
    let t2:f64 = 4.0;
    let c:f64 = 1.609;
    let dm2 = dk2 * c;
    let s2 = dm2 / t2;
    println!("The speed is: {}", s2);

}