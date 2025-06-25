#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(dive, inner(length(1..)))]
    field: &'a [&'a str],
}

fn main() {}
