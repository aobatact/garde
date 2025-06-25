#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(range(10..=100))]
    field: u64,
    #[garde(inner(range(10..=100)))]
    inner: &'a [u64],
}

fn main() {}
