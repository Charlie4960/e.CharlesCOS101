fn main () {
    let t:f64 = 450_000.0;
    let m:f64 = 1_500_000.0;
    let h:f64 = 750_000.0;
    let d:f64 = 2_850_000.0;
    let a:f64 = 250_000.0;

    // Define the quantity as variable
    let tq = 2.0;
    let mq = 1.0;
    let hq = 3.0;
    let dq = 3.0;
    let aq = 1.0;
    let total_quantity = tq+mq+hq+dq+aq;
    println!("The total_quantity is {}", total_quantity );
    
    // calculate the sum by inputting the formula first
    let _sum = (t*tq) + (m*mq) + (h*hq) + (d*dq) + (a*aq);
    println!("The sum is {}", _sum);

    //Calculate the average for the total sales 
    let _average = _sum / total_quantity;
    println!("The average is {}",_average);
}
