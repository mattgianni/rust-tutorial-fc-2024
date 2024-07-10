fn main() {
    let mut x: i32 = 5;
    let r: &mut i32 = &mut x;

    *r += 1;
    *r -= 3;

    // println!("r = {} / x = {}", r, x);
    println!("r = {}", r);
    println!("x = {}", &x);
}
