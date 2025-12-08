#![allow(dead_code)]

#[derive(Debug, garde::Validate)]
struct Inner<'a> {
    #[garde(length(1..))]
    field: &'a str,
    #[garde(inner(length(1..)))]
    inner: &'a [&'a str],
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(dive, length(1..))]
    field: Vec<Inner<'a>>,
    #[garde(length(1..), inner(length(1..)))]
    inner: Vec<&'a str>,
}

fn main() {}
