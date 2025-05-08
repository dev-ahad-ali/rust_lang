use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// default implementation for traits
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize())
}

// trait bound syntax

/* pub fn notify<T: Summary>(item: &T){
    // --- snip ---
} */

// different types that impl Summary

/* pub fn notify(item1: &impl Summary,item2: &impl Summary){
    // --- snip ---
} */

// same type that impl Summary

/* pub fn notify<T: Summary>(item1: &T,item1: &T){
    // --- snip --
} */

// multiple trait bound

/* pub fn notify(item: &(impl Summary + Display)) {
    // --- snip --
} */

/* pub fn notify<T: Summary + Display>(item: &T) {} */

// clearer trait bounds with 'where' clauses

// unclear
/*  fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {} */

pub fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // --- snip ---
}

// return types that impl traits

// this is no allowed right now will discuss in the future
pub fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x : {}", self.x);
        } else {
            println!("The largest number is y : {}", self.y);
        }
    }
}
