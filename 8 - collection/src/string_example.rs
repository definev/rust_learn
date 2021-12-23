pub fn call() {
    let s = String::from("Hé lô mấy cưng!!!");
    // String in rust is utf-8 string
    
    // Because utf-8 string, rust don't know your string is just ascii or utf-8
    // Get bytes in String
    println!("BYTES:");
    for b in s.bytes() {
        println!("{}", b);
    }

    println!("CHARS:");
    for c in s.chars() {
        println!("{}", c);
    }
}
