#![allow(unused)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) {
        (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

struct MultipleTypePoint<T, U> {
    x: T,
    y: U,
}

struct OtherPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> OtherPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: OtherPoint<X2, Y2>) -> OtherPoint<X1, Y2> {
        OtherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 25, 100, 59, 65];
    let largest_number = largest(&number_list);
    println!("the largest number is {largest_number}");

    let char_list = vec!['a', 'y', 'm', 'q'];
    let largest_number = largest(&char_list);
    println!("the largest number is {largest_number}");

    // in struct definitios
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };
    let wont_work = Point { x: 1, y: 2.0 };
    let p = Point { x: 4, y: 2 };
    println!("p.x = {}", p.x());

    let both_integer = MultipleTypePoint { x: 1, y: 2 };
    let both_float = MultipleTypePoint { x: 1.0, y: 2.0 };
    let integer_and_float = MultipleTypePoint { x: 1, y: 2.3 };

    let p1 = OtherPoint { x: 4, y: 2.1 };
    let p2 = OtherPoint { x: "hello", y: "r" };
    let p3 = p1.mixup(p2);
}
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
