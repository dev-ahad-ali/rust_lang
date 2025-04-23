#![allow(unused)]
fn main() {
    let data = "initial content";

    let s1 = String::from("hello");
    let s2 = "hello".to_string();
    let s3 = data.to_string();

    // let hello = String::from(" مكيلعملاسلا");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from(" םולשא");
    // let hello = String::from("नम�त");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    let mut s4 = String::from("hello ");
    let s5 = "world";
    s4.push_str(s5);

    let mut s6 = String::from("lo");
    s6.push('w');

    let s7 = String::from("hello ");
    let s8 = String::from("world");
    let s9 = s7 + &s8; // note: `s7` is moved here and can no longer be used. Because the + operator uses the 'add' method who's signature is: fun add(self, s: &str) -> String

    // concatenate with format!
    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");

    let sc = format!("{s10}-{s11}-{s12}");

    // iterate over the string
    for c in "Зд".chars() {
        print!("{} ", c);
    }
    for b in "Зд".bytes() {
        print!("{} ", b);
    }
}
