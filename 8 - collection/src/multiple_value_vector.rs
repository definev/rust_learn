use std::vec;

enum VariousThing {
    Obj1(String),
    Obj2((u8, String)),
    Obj3(Vec<u8>),
}

pub fn call() {
    let vect = vec![
        VariousThing::Obj1(String::from("The first object")),
        VariousThing::Obj2((125, String::from("Xin chao"))),
        VariousThing::Obj3(vec![1, 23, 1, 3, 4, 1, 41, 4, 5]),
    ];

    match vect.get(1) {
        Some(val) => match val {
            VariousThing::Obj2(tuple) => {
                println!("Obj2 with {} and {}", tuple.0, tuple.1);
            }
            _ => {}
        },
        _ => {}
    }
}
