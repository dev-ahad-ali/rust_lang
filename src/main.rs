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
}
