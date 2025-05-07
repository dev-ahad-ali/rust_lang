#![allow(unused)]
struct Point<T> {
    x: T,
    y: T,
}

struct MultipleTypePoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 25, 100, 59, 65];
    let largest_number = largest(&number_list);
    println!("the largest number is {largest_number}");

    let char_list = vec!['a', 'y', 'm', 'q'];
    let largest_number = largest(&char_list);
    println!("the largest number is {largest_number}");

    // in struct definitions
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };
    let wont_work = Point { x: 1, y: 2.0 };

    let both_integer = MultipleTypePoint { x: 1, y: 2 };
    let both_float = MultipleTypePoint { x: 1.0, y: 2.0 };
    let integer_and_float = MultipleTypePoint { x: 1, y: 2.3 };
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
