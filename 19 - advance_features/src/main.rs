fn main() {}

struct Human {
    name: String,
}

impl Human {
    #[allow(dead_code)]
    fn fly(&self) {
        println!("{} can fly with airplane!", self.name);
    }
}

trait Bird {
    fn fly(&self);
}

impl Bird for Human {
    fn fly(&self) {
        println!("{} can fly with wings!", self.name);
    }
}

trait Chicken {
    fn fly(&self);
}

impl Chicken for Human {
    fn fly(&self) {
        println!("{} cannot fly! Just wacking!", self.name);
    }
}

#[test]
fn advanced_trait_with_self_ref() {
    let human = Human {
        name: String::from("John"),
    };
    human.fly(); // What happens here?
    Bird::fly(&human);
    Chicken::fly(&human);
}

struct Mankind;

impl Mankind {
    // It like static method in Dart
    #[allow(dead_code)]
    fn talk() {
        println!("Hi, I'm a human!");
    }
}

trait Talkative {
    fn talk();
}

impl Talkative for Mankind {
    fn talk() {
        println!("Hi, I'm just wanna talk! So much!!!!!");
    }
}

trait Introvert {
    fn talk();
}

impl Introvert for Mankind {
    fn talk() {
        println!("I don't like to talk ...");
    }
}

#[test]
fn advanced_trait_with_multiple_traits() {
    Mankind::talk();
    <Mankind as Talkative>::talk();
    <Mankind as Introvert>::talk();
}

use core::fmt;

trait BorderPrint: fmt::Display {
    fn border_print(&self) {
        let output = self.to_string();
        let len = output.len();
        let empty = " ".repeat(output.len());
        let border = "*".repeat(len + 4);
        println!("{border}\n* {empty} *\n* {output} *\n* {empty} *\n{border}");
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl BorderPrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[macro_export]
macro_rules! border_print {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        BorderPrint::border_print($($arg)*)
    }};
}

#[test]
fn advance_trait_super_trait() {
    let p1 = Point { x: 1, y: 4 };
    let p2 = Point { x: 31, y: -412 };

    border_print!(&p1);
    border_print!(&p2);
}

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

#[test]
fn example_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = (&mut v).split_at_mut(3);

    assert_eq!(a, vec![1, 2, 3]);
    assert_eq!(b, vec![4, 5, 6]);
}
