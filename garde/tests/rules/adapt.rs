use super::util;

mod test_adapter {
    #![allow(unused_imports)]

    pub use garde::rules::*;

    pub mod length {
        pub use garde::rules::length::*;

        pub mod simple {
            use std::ops::RangeBounds;
            
            pub fn apply<R: RangeBounds<usize>>(v: &str, range: &R) -> garde::Result {
                if !range.contains(&v.len()) {
                    Err(garde::Error::new("my custom error message"))
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(adapt(test_adapter), length(1..))]
    v: &'a str,
}

#[test]
fn alphanumeric_valid() {
    util::check_ok(&[Test { v: "test" }], &())
}

#[test]
fn alphanumeric_invalid() {
    util::check_fail!(&[Test { v: "" }], &())
}
