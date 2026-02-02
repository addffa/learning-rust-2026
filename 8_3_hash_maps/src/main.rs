use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("red"), 8);
    scores.insert(String::from("blue"), 6);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Fav color");
    let field_value = String::from("Green");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    scores.entry(String::from("red")).or_insert(4);
    scores.entry(String::from("yellow")).or_insert(10);
    println!("{scores:#?}");

    let mut plus_one = scores.entry(String::from("red")).or_insert(0);
    *plus_one += 1;
    println!("red: {plus_one}");
    println!("{scores:#?}");
}

