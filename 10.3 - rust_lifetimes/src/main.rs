// Lifetimes is just marking the scope of reference,
// mark for this live long when this function is called

fn main() {
    let a = String::from("ABC");
    let larger;
    {
        let refb: &'static str = "XYZT";
        // This cause error
        larger = largest(a.as_str(), refb);
    }
    println!("Larger: {}", larger);
}

fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
