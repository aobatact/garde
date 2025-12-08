#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(10..=100))]
    field: &'a str,
    #[garde(length(10..=10))]
    field2: &'a str,
    #[garde(length(10..=10))]
    field3: &'a str,
    #[garde(inner(length(10..=100)))]
    inner: &'a [&'a str],
    #[garde(inner(length(10..=10)))]
    inner2: &'a [&'a str],
    #[garde(inner(length(10..=10)))]
    inner3: &'a [&'a str],

    #[garde(length(simple, 1..=1))]
    simple: &'a str,
    #[garde(length(bytes, 1..=1))]
    bytes: &'a str,
    #[garde(length(chars, 1..=1))]
    chars: &'a str,
    #[garde(length(graphemes, 1..=1))]
    graphemes: &'a str,
    #[garde(length(utf16, 1..=1))]
    utf16: &'a str,
}

fn main() {}
