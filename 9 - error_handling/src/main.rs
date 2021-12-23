use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    // panic! macros will stop your program immediately and return a error message on console
    // we need avoid to use it unless we have no choice

    // panic!("Error at main()");
    // println!("What what!"); // Program cannot run in here

    // let v = vec![1, 2, 3, 4];
    // v[99];

    // This maybe cause error so this return Result with Ok or Error
    let file = File::open("hello.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => panic!("Create file failed. {:?}", err),
            },
            other_err => panic!("{:?}", other_err),
        },
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("Content: {}", s),
        Err(_) => panic!("Cannot connect"),
    }
}
