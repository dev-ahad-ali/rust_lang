fn main() {
    let number = 3;

    if number > 5 {
        println!("The number is larger than 5");
    } else {
        println!("The number is smaller than 5");
    }

    if number != 0 {
        println!("The number is something other than 0");
    }

    check_multiple_conditions(6);

    // if else in a statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // let number = if condition { 5 } else { "six" }; this will get an error because all arm must return the same type of value

    println!("The value of number depending on condition is : {number}");
}

fn check_multiple_conditions(x: i32) {
    if x % 4 == 0 {
        println!("The number is divisible by 4");
    } else if x % 3 == 0 {
        println!("The number is divisible by 3");
    } else if x % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3, 2");
    }
}
