use std::thread;

#[allow(unused)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference : {:?} gets : {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference : {:?} gets : {:?}",
        user_pref2, giveaway2
    );

    // types for closure
    let closure_type = |x| x;

    let s = closure_type(String::from("Hello"));
    // let l = closure_type(5); // error when trying with different type

    // capturing reference or moving ownership

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure : {list:?}");
    only_borrows();
    println!("After calling closure : {list:?}");

    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {list2:?}");

    let mut borrow_mutably = || list2.push(7);

    borrow_mutably();
    println!("After calling closure : {list2:?}");

    // giving closure ownership
    let list3 = vec![1, 2, 3];
    thread::spawn(move || println!("From thread: {list3:?}"))
        .join()
        .unwrap();

    // Fn traits and moving captured value out of the closure
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T,
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn main() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{list:#?}, sorted in {num_sort_operations} operations");
    }
}
