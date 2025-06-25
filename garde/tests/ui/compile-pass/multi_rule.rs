#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"), ascii, length(10..=100))]
    field: &'a str,
    #[garde(inner(prefix("test"), ascii, length(10..=100)))]
    inner: &'a [&'a str],
}

fn main() {}
