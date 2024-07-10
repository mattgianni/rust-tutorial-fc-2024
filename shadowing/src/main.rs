fn main() {
    let x = 5;
    let x = x + 1; // shadowed

    {
        let x = x * 2;
        println!("inner scope x = {}", x);
    }

    println!("outter scope x = {}", x);
}
