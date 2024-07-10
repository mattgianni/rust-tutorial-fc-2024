use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let team_name: String = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("The {team_name}'s score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
