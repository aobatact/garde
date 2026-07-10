#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(range(min = 10, max = 100))]
    field: u64,
    #[garde(inner(range(min = 10, max = 100)))]
    inner: &'a [u64],
    #[garde(range(bound = 10..=100))]
    bound: u64,
    #[garde(inner(range(bound = 10..100)))]
    inner_bound: &'a [u64],
}

fn main() {}
