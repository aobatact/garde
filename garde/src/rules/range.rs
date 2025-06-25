//! Range validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(range(min=10,max=100))]
//!     v: u64,
//! }
//! ```
//!
//! The entrypoint is the [`Bounds`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(range(...))]` rule.
//!
//! This trait is implemented for all primitive integer types.

use std::fmt::Display;
use std::ops::RangeBounds;

use crate::error::Error;

#[inline]
fn apply_impl<T, R>(v: &T, range: &R) -> Result<(), Error>
where
    T: PartialOrd + Display,
    R: RangeBounds<T>,
{
    use std::ops::Bound;

    match range.start_bound() {
        Bound::Included(val) => {
            if v < val {
                return Err(Error::new(format!("lower than {val}")));
            }
        }
        Bound::Excluded(val) => {
            if v <= val {
                return Err(Error::new(format!("lower than or equal to {val}")));
            }
        }
        Bound::Unbounded => {}
    };

    match range.end_bound() {
        Bound::Included(val) => {
            if v > val {
                return Err(Error::new(format!("greater than {val}")));
            }
        }
        Bound::Excluded(val) => {
            if v >= val {
                return Err(Error::new(format!("greater than or equal to {val}")));
            }
        }
        Bound::Unbounded => {}
    };

    Ok(())
}

// Trait to extract the inner type for range validation
pub trait RangeValidatable {
    type Inner: PartialOrd + Display;
    fn validate_range<R: RangeBounds<Self::Inner>>(&self, range: &R) -> Result<(), Error>;
}

// Use macros to implement for specific types to avoid conflicts
macro_rules! impl_range_validatable {
    ($($T:ty),*) => {
        $(
            impl RangeValidatable for $T {
                type Inner = $T;

                #[inline]
                fn validate_range<R: RangeBounds<Self::Inner>>(&self, range: &R) -> Result<(), Error> {
                    apply_impl(self, range)
                }
            }

            impl RangeValidatable for Option<$T> {
                type Inner = $T;

                #[inline]
                fn validate_range<R: RangeBounds<Self::Inner>>(&self, range: &R) -> Result<(), Error> {
                    match self {
                        Some(val) => apply_impl(val, range),
                        None => Ok(()),
                    }
                }
            }
        )*
    };
}

impl_range_validatable!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128, f32, f64);

// Main apply function that works with the trait
#[inline]
pub fn apply<V, R>(v: &V, range: &R) -> Result<(), Error>
where
    V: RangeValidatable,
    R: RangeBounds<V::Inner>,
{
    v.validate_range(range)
}

pub trait Bounds: PartialOrd {
    type Size: Copy + Sized + Display;

    const MIN: Self::Size;
    const MAX: Self::Size;

    fn validate_bounds(
        &self,
        lower_bound: Self::Size,
        upper_bound: Self::Size,
    ) -> Result<(), OutOfBounds>;
}

pub enum OutOfBounds {
    Lower,
    Upper,
}

macro_rules! impl_for {
    ($($T:ty),*) => {
        $(
            impl Bounds for $T {
                type Size = $T;

                const MIN: Self::Size = <$T>::MIN;
                const MAX: Self::Size = <$T>::MAX;

                fn validate_bounds(
                    &self,
                    lower_bound: Self::Size,
                    upper_bound: Self::Size,
                ) -> Result<(), OutOfBounds> {
                    if self < &lower_bound {
                        Err(OutOfBounds::Lower)
                    } else if self > &upper_bound {
                        Err(OutOfBounds::Upper)
                    } else {
                        Ok(())
                    }
                }
            }
        )*
    };
}

impl_for!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128, f32, f64);

#[cfg(feature = "rust_decimal")]
impl_for!(rust_decimal::Decimal);

impl<T: Bounds> Bounds for Option<T> {
    type Size = T::Size;

    const MIN: Self::Size = T::MIN;
    const MAX: Self::Size = T::MAX;

    fn validate_bounds(
        &self,
        lower_bound: Self::Size,
        upper_bound: Self::Size,
    ) -> Result<(), OutOfBounds> {
        match self {
            Some(value) => value.validate_bounds(lower_bound, upper_bound),
            None => Ok(()),
        }
    }
}
