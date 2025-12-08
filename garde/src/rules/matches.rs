//! Field matching validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(skip)]
//!     foo: String,
//!     #[garde(matches(foo))]
//!     bar: String,
//! }
//! ```
//!
//! The entrypoint is the [`Matches`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(matches)]` rule.
//!
//! This trait has a blanket implementation for all `T: PartialEq<O>, O`.

use crate::error::{Error, ErrorKind};

pub fn apply<T: Matches<O>, O>(v: &T, (field, value): (&str, &O)) -> Result<(), Error> {
    if !v.validate_matches(value) {
        return Err(Error::from_kind(ErrorKind::FieldMismatch {
            other_field: field.into(),
        }));
    }
    Ok(())
}

pub trait Matches<O> {
    fn validate_matches(&self, other: &O) -> bool;
}

impl<T: PartialEq<O>, O> Matches<O> for T {
    fn validate_matches(&self, other: &O) -> bool {
        self.eq(other)
    }
}
