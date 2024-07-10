fn main() {
    let mut the_vec: Vec<i32> = vec![1, 2, 3];

    for i in 1..5 {
        the_vec.push(i + 3);
    }

    println!("the_vec: {the_vec:?}");

    let i = 2;
    let value = the_vec.get(i);

    match value {
        Some(value) => println!("the {}th value: {}", i + 1, value),
        None => println!("there is no value at the {}th element!", i + 1),
    }
}
