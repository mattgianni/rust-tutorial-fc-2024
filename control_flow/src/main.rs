fn main() {
    let age: u16 = 16;
    if age >= 18 {
        println!("You can vote! and get drafted ...");
    } else {
        println!("No voting for you!");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("The number four goes into six evenly.");
    } else {
        println!("The number four does NOT into six evenly.");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The number: {number}");
}
