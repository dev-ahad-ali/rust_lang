// calculate the area of a rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rec1.width() {
        println!("The rectangle has nonzero width, it is {}", rec1.width);
    }

    println!("The area of the rectangle is {} square pixels", rec1.area())
}
