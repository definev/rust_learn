fn add_one(x: &mut i32) {
    *x = *x + 1;
}

fn add_twice(x: &mut i32, f: fn(&mut i32)) {
    f(x);
    f(x);
    returns_closure()(x);
}

fn returns_closure() -> Box<dyn FnMut(&mut i32)> {
    Box::new(|x| *x = *x + 1)
}

#[test]
fn closure_test() {
    let mut x = 10;
    add_twice(&mut x, add_one);

    println!("{x}");
}

// #[test]
// fn closure() {
//     let answer = add_twice(4, add_one);
//     println!("{answer}\n");
//     let list_of_numbers = vec![1, 2, 3];
//     let list_of_strings: Vec<String> = list_of_numbers //
//         .iter()
//         .map(|i| i.to_string())
//         .collect();
//     let list_some_int: Vec<Result<u32, ()>> = (0u32..40) //
//         .map(Result::Ok)
//         .collect();

//     let vector = vec![1, 2, 3];
//     println!("{:?}", vector);

//     for int in list_some_int {
//         let value = int.unwrap();
//         println!("{value}");
//     }
//     println!("\n");
//     for string in list_of_strings {
//         let value = string;
//         println!("{value}");
//     }
// }
