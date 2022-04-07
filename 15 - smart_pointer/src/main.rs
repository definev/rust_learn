pub mod mock_messenger;
pub mod box_example;
pub mod multiple_ref_count_mutable_ref;

use std::rc::Rc;

use crate::box_example::box_example;

#[allow(dead_code)]
struct CustomSmartPointer {
    data: String,
}

#[allow(dead_code)]
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum RCList {
    Cons(i32, Rc<RCList>),
    Nil,
}

#[allow(dead_code)]
fn custom_drop_example() {
    #[allow(unused_variables)]
    let custom_ptr1 = CustomSmartPointer {
        data: String::from("What?"),
    };
    {
        #[allow(unused_variables)]
        let custom_ptr2 = CustomSmartPointer {
            data: String::from("When?"),
        };
    }
}

#[allow(dead_code)]
fn rc_example() {
    let l1 = Rc::new(RCList::Cons(1, Rc::new(RCList::Nil)));
    println!("Start count after creating l1 = {}", Rc::strong_count(&l1));
    let l2 = RCList::Cons(2, Rc::clone(&l1));
    println!("| AFTER L2: after creating l1 = {}", Rc::strong_count(&l1));
    let l3 = RCList::Cons(2, Rc::clone(&l1));
    println!("| AFTER L3: after creating l1 = {}", Rc::strong_count(&l1));

    println!("L1: {l1:?}");
    println!("L2: {l2:?}");
    println!("L3: {l3:?}");
    a(l1.as_ref());
}

fn a(lx: &RCList) {
    println!("{lx:?}");
}

fn main() {
    box_example();
    // custom_drop_example();
    // rc_example();
}
