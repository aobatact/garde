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

use crate::error::Error;
use std::ops::RangeBounds;

fn check_len(len: usize, min: usize, max: usize) -> Result<(), Error> {
    if len < min {
        Err(Error::new(format!("length is lower than {min}")))
    } else if len > max {
        Err(Error::new(format!("length is greater than {max}")))
    } else {
        Ok(())
    }
}

fn check_len_bounds<R: RangeBounds<usize>>(len: usize, range: &R) -> Result<(), Error> {
    use std::ops::Bound;
    
    match range.start_bound() {
        Bound::Included(&min) => {
            if len < min {
                return Err(Error::new(format!("length is lower than {min}")));
            }
        }
        Bound::Excluded(&min) => {
            if len <= min {
                return Err(Error::new(format!("length is lower than or equal to {min}")));
            }
        }
        Bound::Unbounded => {}
    };

    match range.end_bound() {
        Bound::Included(&max) => {
            if len > max {
                return Err(Error::new(format!("length is greater than {max}")));
            }
        }
        Bound::Excluded(&max) => {
            if len >= max {
                return Err(Error::new(format!("length is greater than or equal to {max}")));
            }
        }
        Bound::Unbounded => {}
    };

    Ok(())
}
