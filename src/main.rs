#[allow(unused)]
struct ImportantExpert<'a> {
    part: &'a str,
}
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r : {r}");

    let string1 = String::from("abcd");

    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
    }

    println!("The longest string is {result}");

    let novel = String::from("Call me ismael, Some years ago ...");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExpert {
        part: first_sentence,
    };

    // static lifetime -> which lives for the entire duration of the program
    let s: &'static str = "I have a static lifetime";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
