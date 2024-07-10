fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
