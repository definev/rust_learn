#[allow(dead_code)]
unsafe fn unsafe_rust() {
    let mut num = 5;

    // raw pointer
    let r1 = &num as *const i32;
    // raw mutable pointer
    let r2 = &mut num as *mut i32;

    // dereference
    let address = 0x12345usize;
    let r = address as *mut i32;

    *r2 = 10;
    *r = 10;
    println!("{}", *r1);
    println!("{}", *r2);
    println!("{}", *r);
}
