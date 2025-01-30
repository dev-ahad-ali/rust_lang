use std::io;

fn main() {
    // Two subsets of data types
    // 1. Scalar
    // 2. Compound

    // --- Scalar types : integers, floating points numbers, Booleans and characters

    // Integer -->

    // floating points number
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("{x}, {y}");

    // Numeric operations

    // addition
    let sum = 5 + 1;

    // subtraction
    let difference = 5 - 1;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 52.2 / 32.3;
    let truncated = -5 / 3; // result in -1

    // remainder
    let remainder = 42 % 5;

    println!("{sum}, {difference}, {product}, {product}, {quotient}, {truncated}, {remainder}");

    // Booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("{t},{f}");

    // character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat_emoji = '😻';

    println!("{c},{z},{heart_eyed_cat_emoji}");

    // ---- Compound types : tuple, array

    // tuples
    let tup: (i32, f64, u8) = (500, 52.7, 1);

    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    let five_hundred = tup.0;
    let float_num = tup.1;
    let one = tup.2;
    println!("{five_hundred},{float_num},{one}");

    // array

    // note : arrays is Rust are fixed length
    let a = [1, 2, 3, 5];
    let x: [i32; 3] = [1, 2, 3]; // with value type and length annotations
    let y = [4; 3]; // this is same as let y = [4,4,4];

    println!("{:?}", (a, x, y));

    println!("Please enter and index number");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered is not a number");

    let element = a[index];

    println!("The value on the element on index : {index} is : {element}");
}
