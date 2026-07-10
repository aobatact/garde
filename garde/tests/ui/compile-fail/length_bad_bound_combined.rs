#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(bound = 1..10, max = 10))]
    field: &'a str,
}

fn main() {}
