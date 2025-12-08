#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length())]  // Empty range - invalid
    field: &'a str,
}

fn main() {}
