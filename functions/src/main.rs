fn main() {
    hello_world();
    tell_height(182);
    human_id("Matthew", 54, 182.0);

    let _x = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };

    println!("Result is: {}", _x);

    let y = add(4, 6);
    println!("y: {}", y);
    println!("Value from function 'add' is: {}", add(4, 6));

    println!("BMI: {:.2}", bmi(81.64, 184.0));
}

fn hello_world() {
    println!("Hello, Rust 🦀!")
}

fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cm.",
        name, age, height
    );
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn bmi(weight: f32, height: f32) -> f32 {
    let height_m = height / 100.0;
    weight / (height_m * height_m)
}
