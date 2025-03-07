fn main() {
    let s: String = String::from("Hello World");
    let s1 = "hello world"; // as string slice

    let word = first_word(&s);
    let word2 = first_word(s1);

    println!("The first word is : {word}");
    println!("The first word is : {word2}");
}

fn first_word(s: &str) -> &str {
    // taking &str string slice type makes it more flexible allowing to take
    // both String and string slice as parameter.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
