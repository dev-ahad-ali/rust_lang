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

    // matching ranges of values only applicable for 'numeric' and 'char' types
    let fo2 = 5;
    match fo2 {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let fo_char = 'c';
    match fo_char {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis : ({x}, {y})"),
    }

    // destructuring nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in y direction {y}");
        }
        Message::Write(text) => {
            println!("The text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r} blue {b} green {g}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h} saturation {s} value {v}");
        } // _ => (),
    }

    // Destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring values in a pattern
    fn fo_fn(_: i32, y: i32) {
        println!("This function only uses y parameter : {y}");
    }
    fo_fn(3, 8);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cannot overwrite an existing customized value");
        }
        _ => setting_value = new_setting_value,
    }

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
    }

    let _fo3 = 3;
    let fo4 = 4;

    let s = Some(String::from("hi"));

    // if let Some(_s) = s {
    //     println!("Found String");
    // }

    if let Some(_) = s {
        println!("Found String");
    }

    println!("{s:?}");

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is : {x}"),
    }
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // match guard
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let fo5 = 4;
    let fo_bool = false;

    match fo5 {
        4 | 5 | 6 if fo_bool => println!("yes"),
        _ => println!("no"),
    }

    // '@' at bindings
    enum Text {
        Hello { id: i32 },
    }

    let foo_msg = Text::Hello { id: 5 };

    match foo_msg {
        Text::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range : {id_variable}"),
        Text::Hello { id: 10..=12 } => println!("Found an id in another range "),
        Text::Hello { id } => println!("Found some other id: {id}"),
    }
}
