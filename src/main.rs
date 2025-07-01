fn main() {
    // matching literals
    let fo = 1;
    match fo {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end , x={x:?} and y={y}");

    // multiple patterns
    let fo1 = 1;
    match fo1 {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
