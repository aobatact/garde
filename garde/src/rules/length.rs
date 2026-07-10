//! Length validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(length(min=1, max=100))]
//!     v: String,
//! }
//! ```
//!
//! The concept of "length" is somewhat complicated, especially for strings. Therefore, the `length` rule currently supports different modes:
//! - [`Simple`][simple::Simple], which is the default
//! - [`Bytes`][bytes::Bytes]
//! - [`Chars`][chars::Chars]
//! - [`Graphemes`][graphemes::Graphemes]
//! - [`Utf16CodeUnits`][utf16::Utf16CodeUnits]
//!
//! The mode is configured on the `length` rule:
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(
//!         length(graphemes, min=1, max=25),
//!         length(bytes, min=1, max=100),
//!     )]
//!     v: String,
//! }
//! ```
//!
//! Instead of `min`/`max`, an arbitrary [`RangeBounds`][std::ops::RangeBounds]
//! expression may be provided via `bound`. This is mutually exclusive with
//! `min`, `max`, and `equal`:
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(length(bound = 1..=100))]
//!     v: String,
//! }
//! ```
//!
//! Here's what implementing the trait for a custom string-like type might look like:
//! ```rust
//! #[repr(transparent)]
//! struct MyString(String);
//!
//! impl garde::rules::length::HasSimpleLength for MyString {
//!     fn length(&self) -> usize {
//!         self.0.len()
//!     }
//! }
//! ```
//!
//! See each trait for more information.
//!

pub mod bytes;
pub use bytes::HasBytes;

pub mod chars;
pub use chars::HasChars;

#[cfg(feature = "unicode")]
pub mod graphemes;
#[cfg(feature = "unicode")]
pub use graphemes::HasGraphemes;

pub mod simple;
pub use simple::HasSimpleLength;

pub mod utf16;
pub use utf16::HasUtf16CodeUnits;

use std::ops::{Bound, RangeBounds};

use crate::error::Error;

fn check_len(len: usize, min: usize, max: usize) -> Result<(), Error> {
    if len < min {
        Err(Error::new(i18n!(length_lower_than, min)))
    } else if len > max {
        Err(Error::new(i18n!(length_greater_than, max)))
    } else {
        Ok(())
    }
}

/// Convert an arbitrary [`RangeBounds<usize>`] into the inclusive `(min, max)`
/// pair used by [`check_len`].
///
/// Because length is measured in discrete `usize` units, exclusive bounds are
/// mapped exactly to inclusive ones by shifting them by one (saturating at the
/// `usize` limits).
fn bounds_to_min_max<R: RangeBounds<usize>>(bounds: &R) -> (usize, usize) {
    let min = match bounds.start_bound() {
        Bound::Included(&min) => min,
        Bound::Excluded(&min) => min.saturating_add(1),
        Bound::Unbounded => usize::MIN,
    };
    let max = match bounds.end_bound() {
        Bound::Included(&max) => max,
        Bound::Excluded(&max) => max.saturating_sub(1),
        Bound::Unbounded => usize::MAX,
    };
    (min, max)
}
