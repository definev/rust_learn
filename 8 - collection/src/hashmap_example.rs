use std::collections::HashMap;

pub fn call() {
    let yellow = String::from("Yellow");
    let blue = String::from("Blue");
    let mut map = HashMap::new();

    map.insert(yellow, 500);
    map.insert(blue, 100);

    let key_name = String::from("Blue");

    match map.get(&key_name) {
        Some(value) => println!("Value of {}: {}", key_name, value),
        None => println!("Cannot find any thing!")
    }

    let word = String::from("Wonderful world world neva end");
    let mut w_map = HashMap::new();

    for w in word.split_whitespace() {
        let count = w_map.entry(w).or_insert(0);
        *count += 1;
    }

    println!("{:?}", w_map);
}
