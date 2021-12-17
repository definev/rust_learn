fn main() {
    let s = String::from("Bui Dai Duong");
    let ho = &s[0..3];
    let dem = &s[4..7];
    let ten = &s[8..13];

    println!("{} {} {}", ho, dem, ten);
}