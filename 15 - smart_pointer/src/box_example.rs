#[derive(Debug)]
pub enum BoxList {
    Nil,
    Cons(i32, Box<BoxList>),
}

// Can't compile
// #[derive(Debug)]
// pub enum NormalList {
//     Nil,
//     Cons(i32, NormalList),
// }

pub fn box_example() {
    let l1 = BoxList::Cons(
        31,
        Box::new(
            BoxList::Cons(
                521,
                Box::new(BoxList::Cons(231, Box::new(BoxList::Nil))), //
            ), //
        ),
    );
    println!("{l1:?}");

    // let pre_norm_l1 = NormalList::Cons(4134, NormalList::Nil); //
    // let pre_norm_l2 = NormalList::Cons(4134, pre_norm_l1); //

    // let norm_l1 = NormalList::Cons(13, pre_norm_l2);

    // println!("{pre_norm_l1:?}");
}
