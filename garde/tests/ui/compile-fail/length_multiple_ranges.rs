#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(1.., 5..=10))]  // Multiple ranges - invalid
    multiple_ranges: &'a str,
}

fn main() {}
