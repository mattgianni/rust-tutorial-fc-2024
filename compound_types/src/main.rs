fn main() {
    // i32 array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);

    // string array
    println!();
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruit array: {:?}", fruits);

    println!("Fruit array[0]: {}", fruits[0]);
    println!("Fruit array[1]: {}", fruits[1]);
    println!("Fruit array[2]: {}", fruits[2]);

    // TUPLES
    println!();
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    // mixed tuple
    let my_mix_tuple: (&str, i32, bool, [i32; 5]) = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("Mixed tuple: {:?}", my_mix_tuple);

    // slices
    let number_slices: &[i32; 5] = &[1, 2, 3, 4, 5];
    println!("Number slice: {:?}", number_slices);

    let animal_slices: &[&str; 3] = &["Python", "Crab", "Wolf"];
    println!("Animal slice: {:?}", animal_slices);

    let book_slices: &[String; 3] = &[
        "Bible".to_string(),
        "Elements".to_string(),
        "Common Sense".to_string(),
    ];
    println!("Book slice: {:?}", book_slices);

    //////////////////////////////////////////////////////////////////////////////////////////
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    let string: String = String::from("Hello, world!");
    let slice: &str = &string[0..5];
    println!("Source string: {}", string);

    print(slice);
    println!("Range value: {:?}", 0..5);
}

pub fn print(slice: &str) {
    println!("Slice: {}", slice);
}
