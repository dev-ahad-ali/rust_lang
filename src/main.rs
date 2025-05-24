use std::ops::Deref;

use crate::List::{Cons, Nil};
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop CustomSmartPointer with data : {}", self.data);
    }
}
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let y2 = Box::new(x); // copied the value of x on the heap
    let y3 = MyBox::new(x);
    let m = MyBox::new(String::from("world"));
    let c = CustomSmartPointer {
        data: String::from("some stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("more stuff"),
    };

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y2);
    assert_eq!(5, *y3);
    hello(&m);
    println!("CustomSmartPointer created with : {:?}, {:?}", c, d);
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
