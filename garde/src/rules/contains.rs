//! Substring validation.
//!
//! ```rust
//! const STR: &str = "test";
//!
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(contains("test"))]
//!     v: String,
//!     #[garde(contains(STR))]
//!     w: String,
//! }
//! ```
//!
//! The entrypoint is the [`Contains`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(contains)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use super::AsStr;
use crate::error::{Error, StandardErrorCode};

pub fn apply<T: Contains>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.validate_contains(pat) {
        return Err(Error::with_code(
            format!("does not contain \"{pat}\""),
            StandardErrorCode::ContainsNotFound,
        ));
    }
    Ok(())
}

pub fn apply_with_code<T: Contains>(v: &T, (pat,): (&str,), code: &str) -> Result<(), Error> {
    if !v.validate_contains(pat) {
        return Err(Error::with_custom_code(
            format!("does not contain \"{pat}\""),
            code,
        ));
    }
    Ok(())
}

pub trait Contains {
    fn validate_contains(&self, pat: &str) -> bool;
}

impl<T: AsStr> Contains for T {
    fn validate_contains(&self, pat: &str) -> bool {
        self.as_str().contains(pat)
    }
}

impl<T: Contains> Contains for Option<T> {
    fn validate_contains(&self, pat: &str) -> bool {
        match self {
            Some(value) => value.validate_contains(pat),
            None => true,
        }
    }
}
