
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
    ($($arg:tt)+) => {
        BorderPrint::border_print($($arg)?);
    };
}

#[test]
fn advance_trait_super_trait() {
    let p1 = Point { x: 1, y: 4 };
    let p2 = Point { x: 31, y: -412 };

    border_print!(&p1);
    border_print!(&p2);
}