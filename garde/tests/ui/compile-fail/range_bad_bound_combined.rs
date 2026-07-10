#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test {
    #[garde(range(bound = 1..10, min = 1))]
    field: u64,
}

fn main() {}
