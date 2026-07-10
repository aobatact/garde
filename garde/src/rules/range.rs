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
//! Instead of `min`/`max`, an arbitrary [`RangeBounds`][std::ops::RangeBounds]
//! expression may be provided via `bound`. This is mutually exclusive with
//! `min`, `max`, and `equal`:
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(range(bound = 10..=100))]
//!     v: u64,
//! }
//! ```
//!
//! The entrypoint is the [`Bounds`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(range(...))]` rule.
//!
//! This trait is implemented for all primitive integer types.

use std::fmt::Display;
use std::ops::{Bound, RangeBounds};

use crate::error::Error;

#[inline]
pub fn apply<T: Bounds>(
    v: &T,
    (min, max): (Option<T::Size>, Option<T::Size>),
) -> Result<(), Error> {
    let min = min.unwrap_or(T::MIN);
    let max = max.unwrap_or(T::MAX);
    if let Err(e) = v.validate_bounds(min, max) {
        match e {
            OutOfBounds::Lower => return Err(Error::new(i18n!(range_lower_than, &min))),
            OutOfBounds::Upper => return Err(Error::new(i18n!(range_greater_than, &max))),
        }
    }
    Ok(())
}

#[inline]
pub fn apply_bounds<T, R>(v: &T, (bounds,): (R,)) -> Result<(), Error>
where
    T: Bounds,
    R: RangeBounds<T::Size>,
{
    match v.validate_range_bounds(&bounds) {
        Ok(()) => Ok(()),
        Err(OutOfBounds::Lower) => {
            let min = match bounds.start_bound() {
                Bound::Included(min) | Bound::Excluded(min) => min as &dyn Display,
                Bound::Unbounded => &T::MIN,
            };
            Err(Error::new(i18n!(range_lower_than, min)))
        }
        Err(OutOfBounds::Upper) => {
            let max = match bounds.end_bound() {
                Bound::Included(max) | Bound::Excluded(max) => max as &dyn Display,
                Bound::Unbounded => &T::MAX,
            };
            Err(Error::new(i18n!(range_greater_than, max)))
        }
    }
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

    /// Validate that `self` lies within `bounds`, honoring the inclusive or
    /// exclusive nature of each end.
    ///
    /// The default implementation treats exclusive bounds as inclusive (it
    /// forwards to [`validate_bounds`][Bounds::validate_bounds]). All
    /// implementations provided by `garde` override this to respect
    /// exclusivity, so custom implementors who care about exclusive bounds
    /// should override it too.
    fn validate_range_bounds<R>(&self, bounds: &R) -> Result<(), OutOfBounds>
    where
        R: RangeBounds<Self::Size>,
    {
        let lower_bound = match bounds.start_bound() {
            Bound::Included(&bound) | Bound::Excluded(&bound) => bound,
            Bound::Unbounded => Self::MIN,
        };
        let upper_bound = match bounds.end_bound() {
            Bound::Included(&bound) | Bound::Excluded(&bound) => bound,
            Bound::Unbounded => Self::MAX,
        };
        self.validate_bounds(lower_bound, upper_bound)
    }
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

                fn validate_range_bounds<R>(&self, bounds: &R) -> Result<(), OutOfBounds>
                where
                    R: RangeBounds<Self::Size>,
                {
                    match bounds.start_bound() {
                        Bound::Included(lower) if self < lower => return Err(OutOfBounds::Lower),
                        Bound::Excluded(lower) if self <= lower => return Err(OutOfBounds::Lower),
                        _ => {}
                    }
                    match bounds.end_bound() {
                        Bound::Included(upper) if self > upper => return Err(OutOfBounds::Upper),
                        Bound::Excluded(upper) if self >= upper => return Err(OutOfBounds::Upper),
                        _ => {}
                    }
                    Ok(())
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

    fn validate_range_bounds<R>(&self, bounds: &R) -> Result<(), OutOfBounds>
    where
        R: RangeBounds<Self::Size>,
    {
        match self {
            Some(value) => value.validate_range_bounds(bounds),
            None => Ok(()),
        }
    }
}
