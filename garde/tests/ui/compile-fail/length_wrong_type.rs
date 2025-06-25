#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length("invalid_syntax"))]  // Invalid range syntax
    invalid_syntax: &'a str,
}

fn main() {}
