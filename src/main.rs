fn main() {
    // conditional if let expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as background color");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color");
        } else {
            println!("Using orange as background color");
        }
    } else {
        println!("Using blue as background color");
    }

    // while let conditional loop
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    // for loops
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index : {index}")
    }

    // let patterns
    let (x, y, z) = (1, 2, 3);
}
