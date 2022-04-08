use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum MutList {
    Nil,
    Cons(Rc<RefCell<i32>>, Box<MutList>),
}

#[allow(dead_code)]
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

    match &ml1 {
        MutList::Cons(data, _) => {
            *data.borrow_mut() = 213;
        }
        _ => {}
    }
    println!("{ml1:?}");
    println!("{ml2:?}");
    println!("{data:?}");
}

#[cfg(test)]
mod tests {
    use super::multiple_rc_mut;

    #[test]
    fn multiple_rc_mut_test() {
        multiple_rc_mut();
    }
}
