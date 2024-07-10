fn divide_option(numerator: f64, demoninator: f64) -> Option<f64> {
    if demoninator == 0.0 {
        None
    } else {
        Some(numerator / demoninator)
    }
}

fn divide_result(numerator: f64, demoninator: f64) -> Result<f64, String> {
    if demoninator == 0.0 {
        Err(String::from("divide by zero error."))
    } else {
        Ok(numerator / demoninator)
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    let option = divide_option(10.0, 1.0);
    match option {
        Some(x) => println!("Result: {}", x),
        None => println!("No result!"),
    }

    let result = divide_result(10.0, 0.0);
    match result {
        Ok(x) => println!("Result: {x}"),
        Err(msg) => println!("{msg}"),
    }
}
