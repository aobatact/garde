#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(10..=))]  // Invalid range syntax: missing end
    invalid_range: &'a str,
}

fn main() {}
