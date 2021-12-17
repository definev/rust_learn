fn main() {
    // Normal match and ignore the None case
    let some_value = Some(3u8);
    match some_value {
        Some(value) => println!("{}", value),
        _ => {}
    }

    // The same as but more verbose
    if let Some(value) = some_value {
        println!("{}", value);
    }
}
