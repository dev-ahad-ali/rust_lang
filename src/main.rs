fn main() {
    let s = String::from("hello");
    let mut s1 = String::from("Hello");
    let mut r = String::from("Ref");

    let length = calculate_length(&s);

    println!("The size of {s} is {length}");

    change(&mut s1);

    // simultaneous mutable reference are not allowed
    // let r1 = &mut r;
    // let r2 = &mut r;

    {
        let r1 = &mut r;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems. allowing for multiple mutable references, just not simultaneous ones

    let r2 = &mut r;
    println!("{}", r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}
