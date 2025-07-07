fn main() {
    // Associated types
    struct Counter {}
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            // snip---
            Some(32)
        }
    }
}
