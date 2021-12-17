fn main() {
    let decimal = 100_000;
    let hexa = 0xee_2e121;
    let octa = 0o77_1_23_412_21;
    let binary = 0b1111_0000_1111_0000;

    println!("Decimal: {}", decimal);
    println!("Hexa: {}", hexa);
    println!("Octa: {}", octa);
    println!("Binary: {}", binary);

    let flp = 0.1 + 0.2;
    println!("Floating point: {}", flp);
    let flp = 0.2 + 0.1;
    println!("Floating point: {}", flp);

    // Character
    let c = 'c';
    let z = 'ℤ';
    let hearted_cat_eyes: char = '😻';

    let sentence = "This is a 😻";
    let cat = match sentence.chars().nth(10) {
        Some(cat) => cat,
        None => 'n',
    };

    println!("{}\n{}\n{}\n", c, z, hearted_cat_eyes);
    println!("{}\n{}", sentence, cat);

    // Tuple types
    let cordinate: (i32, i32) = (0, 0);
    println!("Cordinate: ({}, {})\n", cordinate.0, cordinate.1);

    // Array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let numbers: [i32; 5] = [0, 1, 2, 3, 4];
    let aug = months[7];
    println!("Numbers: {}\nAug: {}\n", numbers[4], aug);
}
