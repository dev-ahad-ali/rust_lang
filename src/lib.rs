use std::fmt::Display;

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
// impl Summary for NewsArticle {}

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
