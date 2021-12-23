fn main() {
    ex_1();
    ex_2();
}

fn ex_1() {
    let v1 = vec![34, 214, 12, 45, 125, 214];
    let (v1, larget_v1) = get_largest(v1);
    println!("{} is larget in {:?}", larget_v1, v1);
}

// We need specify trait of T because T must comparable
fn get_largest<T: PartialOrd + Copy>(v: Vec<T>) -> (Vec<T>, T) {
    let mut larget = v[0];
    for ele in &v {
        if larget < *ele {
            larget = *ele;
        }
    }
    (v, larget)
}

// Generic can specify with many type
struct Point<T, U> {
    x: T,
    y: U,
}

fn ex_2() {
    let p1 = Point { x: 4, y: 10.0 };
    println!("Cordinate: ({}, {})", p1.x, p1.y);
}
