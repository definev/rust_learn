fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length_and_mut(&mut s1);
    println!("{}, world! with hello is {} characters!", s1, len);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    {
        let r3 = &mut s; // no problem
        r3.push_str(", world!");
    }
    println!("{}", s);
}

fn calculate_length_and_mut(s1_ref: &mut String) -> usize {
    s1_ref.push_str(", world!");
    s1_ref.len()
}
