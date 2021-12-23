pub fn call() {
    let mut v: Vec<u8> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];
    // Access to elements
    // The immutable reference
    let third = &v2[1];
    println!("Third element: {}", third);

    // Because of heap cannot know your vector size so we can get the value with the safe way
    match v2.get(3) {
        Some(value) => println!("The value of 3: {}", value),
        None => println!("Not found!"),
    }

    // Mutable vector
    let mut v3 = vec![1, 2, 3];
    
    // Mutable reference
    for i in &mut v3 {
        // * is dereference
        *i += 50;
    }

    for i in &v3 {
        println!("V3 element: {}", i);
    }
}