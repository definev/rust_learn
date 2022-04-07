use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum MutList {
    Nil,
    Cons(Rc<RefCell<i32>>, Box<MutList>),
}

#[warn(dead_code)]
fn multiple_rc_mut() {
    let data = Rc::new(RefCell::new(123));
    let ml1 = MutList::Cons(data.clone(), Box::new(MutList::Nil));
    let ml2 = MutList::Cons(data.clone(), Box::new(MutList::Nil));

    match &ml1 {
        MutList::Cons(data, _) => {
            *data.borrow_mut() = 1234;
        }
        _ => {}
    }

    println!("{data:?}");
    println!("{ml1:?}");
    println!("{ml2:?}");
}

#[cfg(test)]
mod tests {
    use crate::multiple_rc_mut::multiple_rc_mut;

    #[test]
    fn multiple_rc_mut_test() {
        multiple_rc_mut();
    }
}