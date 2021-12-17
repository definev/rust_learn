fn another_ffi() {
    println!("Foreign again function!");
}

fn main() {
    println!("Hello, world!");
    println!("Sum: {}", sum(3, 45));
}

fn ffi() {
    println!("Foreign function!");
    another_ffi();
}

fn sum(a: i32, b: i32) -> i32 {
    let ffi_fn = ffi;
    ffi_fn();
    let x = if a > 5 { 6 } else { 3 };
    return x;
}
