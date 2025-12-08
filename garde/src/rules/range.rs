//! Range validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(range(10..=100))]
//!     v: u64,
//! }
//! ```
//!
//! The entrypoint is the [`Bounds`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(range(...))]` rule.
//!
//! This trait is implemented for all primitive integer types.

use std::fmt::Display;
use std::ops::RangeBounds;

use crate::error::{Error, ErrorKind, RangeBound};

// Main apply function that works with the trait
#[inline]
pub fn apply<V, R, T>(v: &V, range: &R) -> Result<(), Error>
where
    V: RangeValidatable<T, R>,
    R: RangeBounds<T>,
    T: PartialOrd + Display,
{
    v.validate_range(range)
}

// Trait to extract the inner type for range validation
pub trait RangeValidatable<T: PartialOrd + Display, R: RangeBounds<T>> {
    fn validate_range(&self, range: &R) -> Result<(), Error>;
}

impl<T: PartialOrd + Display, R: RangeBounds<T>> RangeValidatable<T, R> for T {
    fn validate_range(&self, range: &R) -> Result<(), Error> {
        use std::ops::Bound;

        match range.start_bound() {
            Bound::Included(val) => {
                if self < val {
                    return Err(Error::from_kind(ErrorKind::RangeTooLow {
                        min: RangeBound::Inclusive(val.to_string().into()),
                        actual: self.to_string().into(),
                    }));
                }
            }
            Bound::Excluded(val) => {
                if self <= val {
                    return Err(Error::from_kind(ErrorKind::RangeTooLow {
                        min: RangeBound::Exclusive(val.to_string().into()),
                        actual: self.to_string().into(),
                    }));
                }
            }
            Bound::Unbounded => {}
        };

        match range.end_bound() {
            Bound::Included(val) => {
                if self > val {
                    return Err(Error::from_kind(ErrorKind::RangeTooHigh {
                        max: RangeBound::Inclusive(val.to_string().into()),
                        actual: self.to_string().into(),
                    }));
                }
            }
            Bound::Excluded(val) => {
                if self >= val {
                    return Err(Error::from_kind(ErrorKind::RangeTooHigh {
                        max: RangeBound::Exclusive(val.to_string().into()),
                        actual: self.to_string().into(),
                    }));
                }
            }
            Bound::Unbounded => {}
        };

        Ok(())
    }
}

impl<T: PartialOrd + Display, U: RangeBounds<T>> RangeValidatable<T, U> for Option<T> {
    fn validate_range(&self, range: &U) -> Result<(), Error> {
        match self {
            Some(val) => val.validate_range(range),
            None => Ok(()),
        }
    }
}
