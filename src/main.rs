// calculate the area of a rectangle
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", area(&rec1))
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
