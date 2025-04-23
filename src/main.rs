fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);

    let v1: Vec<i32> = vec![1, 2, 3, 4];

    let mut v2: Vec<i32> = vec![1, 2, 3, 4];

    let third_element: &i32 = &v1[2];
    println!("Third element:{third_element}");

    let third_element: Option<&i32> = v1.get(2);

    match third_element {
        Some(element) => println!("Third element:{element}"),
        None => println!("There is no third element"),
    }

    // iterate over the vector
    for i in &v1 {
        println!("Value:{}", i);
    }
    for i in &mut v2 {
        *i += 50;
    }
    // storing different types in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Row: {:#?}", row);
}
