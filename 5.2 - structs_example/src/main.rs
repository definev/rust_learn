fn main() {
    // method1();
    // method2();
    // method3();
    method_with_trait();
}

fn area_m1(height: u32, width: u32) -> u32 {
    height * width
}

// Method 1: Use raw value to calculate
fn method1() {
    let width: u32 = 20;
    let height: u32 = 30;

    println!("The area of rectangle is {}.", area_m1(height, width));
}

fn area_m2(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

// Method 2: Use tuple types
fn method2() {
    let rectangle: (u32, u32) = (30, 40);

    println!("The area of rectangle is {}.", area_m2(rectangle));
}
#[derive(Debug, Clone)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
}

impl Rectangle {
    fn particular(&self, scale: u32) -> u32 {
        return (self.height + self.width) * scale;
    }
}

fn area_m3(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

// Method 3: Use struct
fn method3() {
    let rectangle = Rectangle {
        height: 30,
        width: 60,
    };

    println!("The area of rectangle is {}.", area_m3(&rectangle));
}

fn method_with_trait() {
    let scale = 10;
    let rect = Rectangle {
        height: dbg!(20 * scale),
        width: 60,
    };

    println!("Area: {}", rect.area());
    println!("Scale: {}", rect.particular(3));

    // println!("Rect is {:#?}", rect);
    dbg!(&rect);
}
