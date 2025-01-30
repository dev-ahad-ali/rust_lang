fn main() {
    // Rust has 3 kinds of loops : loop, while, for

    // loop_keyword();
    value_form_loop();
    labeled_loop();
    while_loop();
    for_loop();
    for_loop_countdown();
}

// fn loop_keyword() {
//     // this will be a infinity loop util you stop the program with ctrl + c
//     loop {
//         println!("again!");
//     }
// }

fn value_form_loop() {
    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is : {result}");
}

fn labeled_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("Count = {count}");

        let mut remaining = 10;

        loop {
            println!("Reaming = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count : {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    for num in numbers {
        println!("The value is : {}", num);
    }
}

fn for_loop_countdown() {
    for number in (1..4).rev() {
        println!("The value in countdown is : {number}");
    }
}
