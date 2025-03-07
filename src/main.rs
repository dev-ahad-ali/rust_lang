fn main() {
    let s: String = String::from("Hello World");
    let s1 = "hello world"; // as string slice

    let word = first_word(&s);
    let word2 = first_word(s1);

    println!("The first word is : {word}");
    println!("The first word is : {word2}");

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]); // Asserts that two expressions are equal to each other.
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
