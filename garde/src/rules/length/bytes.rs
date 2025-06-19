//! Implemented by types for which we can retrieve the number of bytes.
//!
//! See also: [`chars` on `str`](https://doc.rust-lang.org/std/primitive.str.html#method.chars).

use crate::error::Error;
use std::ops::RangeBounds;

pub fn apply_bounds<T: Bytes, R: RangeBounds<usize>>(v: &T, range: &R) -> Result<(), Error> {
    v.validate_num_bytes_bounds(range)
}

pub trait Bytes {
    fn validate_num_bytes_bounds<R: RangeBounds<usize>>(&self, range: &R) -> Result<(), Error>;
}

impl<T: HasBytes> Bytes for T {
    fn validate_num_bytes_bounds<R: RangeBounds<usize>>(&self, range: &R) -> Result<(), Error> {
        super::apply_bounds(self.num_bytes(), range)
    }
}

impl<T: Bytes> Bytes for Option<T> {
    fn validate_num_bytes_bounds<R: RangeBounds<usize>>(&self, range: &R) -> Result<(), Error> {
        match self {
            Some(v) => v.validate_num_bytes_bounds(range),
            None => Ok(()),
        }
    }
}

pub trait HasBytes {
    fn num_bytes(&self) -> usize;
}

macro_rules! impl_via_len {
    ($(in<$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasBytes for $T {
            fn num_bytes(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_via_len!(std::string::String);
impl_via_len!(in<'a> &'a std::string::String);
impl_via_len!(in<'a> &'a str);
impl_via_len!(in<'a> std::borrow::Cow<'a, str>);
impl_via_len!(std::rc::Rc<str>);
impl_via_len!(std::sync::Arc<str>);
impl_via_len!(std::boxed::Box<str>);
impl_via_len!(in<'a> &'a [u8]);
impl_via_len!(std::rc::Rc<[u8]>);
impl_via_len!(std::sync::Arc<[u8]>);
impl_via_len!(std::boxed::Box<[u8]>);
impl_via_len!(std::vec::Vec<u8>);

impl<const N: usize> HasBytes for [u8; N] {
    fn num_bytes(&self) -> usize {
        self.len()
    }
}
