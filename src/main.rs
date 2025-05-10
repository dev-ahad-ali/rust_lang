use ::std::fmt::Display;
fn main() {
    let string1 = "I'm the longest string";
    let string2 = "I'm not";
    let announce = "Let's begin";

    longest_announcement(string1, string2, announce);
}

fn longest_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    print!("Announcement: {ann}");
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}
