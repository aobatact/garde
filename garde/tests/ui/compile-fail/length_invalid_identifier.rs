#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(not_a_range))]  // Invalid: not a range expression
    field: &'a str,
}

fn main() {}
