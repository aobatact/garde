//! Length validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(length(1..=100))]
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
//!         length(graphemes, 1..=25),
//!         length(bytes, 1..=100),
//!     )]
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

use crate::error::{Error, ErrorKind, LengthBound};
use std::ops::RangeBounds;

pub fn apply<R: RangeBounds<usize>>(len: usize, range: &R) -> Result<(), Error> {
    use std::ops::Bound;

    match range.start_bound() {
        Bound::Included(&min) => {
            if len < min {
                return Err(Error::from_kind(ErrorKind::LengthTooShort {
                    min: LengthBound::Inclusive(min),
                    actual: len,
                }));
            }
        }
        Bound::Excluded(&min) => {
            if len <= min {
                return Err(Error::from_kind(ErrorKind::LengthTooShort {
                    min: LengthBound::Exclusive(min),
                    actual: len,
                }));
            }
        }
        Bound::Unbounded => {}
    };

    match range.end_bound() {
        Bound::Included(&max) => {
            if len > max {
                return Err(Error::from_kind(ErrorKind::LengthTooLong {
                    max: LengthBound::Inclusive(max),
                    actual: len,
                }));
            }
        }
        Bound::Excluded(&max) => {
            if len >= max {
                return Err(Error::from_kind(ErrorKind::LengthTooLong {
                    max: LengthBound::Exclusive(max),
                    actual: len,
                }));
            }
        }
        Bound::Unbounded => {}
    };

    Ok(())
}
