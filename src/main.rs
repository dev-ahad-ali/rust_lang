fn main() {
    another_function(3);
    print_labeled_measurement(5, 'h');

    // this is an example of expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is : {y}");

    let z = five();
    println!("The value of z is : {z}");

    let p = plus_one(5);
    println!("The value of p is : {p}");
}

fn another_function(x: i32) {
    println!("The value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

// line without semicolon ; (expression) will be the return value of the function or return key can be used for explicitly return a value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
