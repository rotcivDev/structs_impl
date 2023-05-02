#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1: {:#?}", rect1);

    let rect2 = Rectangle {
        height: 20,
        width: 25,
    };
    println!("rect2: {:#?}", rect2);

    println!("Can rect1 hold rect2?\n{}", rect1.can_hold(&rect2));

    let square1 = Rectangle::square(20);

    println!("square1: {:#?}", square1);

    println!("The area of the rect1 is {} square pixels.", rect1.area());
    println!(
        "The area of the square1 is {} square pixels.",
        square1.area()
    );
}
