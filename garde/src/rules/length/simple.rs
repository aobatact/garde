//! Implemented by types which have a known length.
//!
//! The meaning of "length" depends on the type.
//! For example, the length of a `String` is defined as the number of _bytes_ it stores.

use crate::error::Error;
use std::ops::RangeBounds;

pub fn apply<T: Simple<R>, R: RangeBounds<usize>>(v: &T, range: &R) -> Result<(), Error> {
    v.validate_length(range)
}

pub fn apply_with_code<T: HasSimpleLength, R: RangeBounds<usize>>(v: &T, range: &R, code: &str) -> Result<(), Error> {
    super::apply_with_code(v.length(), range, code)
}

pub trait Simple<R: RangeBounds<usize>> {
    fn validate_length(&self, range: &R) -> Result<(), Error>;
}

impl<T: HasSimpleLength, R: RangeBounds<usize>> Simple<R> for T {
    fn validate_length(&self, range: &R) -> Result<(), Error> {
        super::apply(self.length(), range)
    }
}

impl<T, R: RangeBounds<usize>> Simple<R> for Option<T>
where
    T: Simple<R>,
{
    fn validate_length(&self, range: &R) -> Result<(), Error> {
        match self {
            Some(v) => v.validate_length(range),
            None => Ok(()),
        }
    }
}

pub trait HasSimpleLength {
    fn length(&self) -> usize;
}

macro_rules! impl_via_bytes {
    ($(in<$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasSimpleLength for $T {
            fn length(&self) -> usize {
                use super::bytes::HasBytes as _;
                self.num_bytes()
            }
        }
    };
}

impl_via_bytes!(std::string::String);
impl_via_bytes!(in<'a> &'a std::string::String);
impl_via_bytes!(in<'a> &'a str);
impl_via_bytes!(in<'a> std::borrow::Cow<'a, str>);
impl_via_bytes!(std::rc::Rc<str>);
impl_via_bytes!(std::sync::Arc<str>);
impl_via_bytes!(std::boxed::Box<str>);

macro_rules! impl_via_len {
    (in<$lifetime:lifetime, $($generic:ident),*> $T:ty) => {
        impl<$lifetime, $($generic),*> HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    (in<$($generic:ident),*> $T:ty) => {
        impl<$($generic),*> HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    (in<$lifetime:lifetime> $T:ty) => {
        impl<$lifetime> HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    ($T:ty) => {
        impl HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_via_len!(in<T> Vec<T>);
impl_via_len!(in<'a, T> &'a Vec<T>);
impl_via_len!(in<'a, T> &'a [T]);

impl<const N: usize, T, R: RangeBounds<usize>> Simple<R> for [T; N] {
    fn validate_length(&self, range: &R) -> Result<(), Error> {
        super::apply(self.len(), range)
    }
}

impl<const N: usize, T, R: RangeBounds<usize>> Simple<R> for &[T; N] {
    fn validate_length(&self, range: &R) -> Result<(), Error> {
        super::apply(self.len(), range)
    }
}

impl_via_len!(in<K, V, S> std::collections::HashMap<K, V, S>);
impl_via_len!(in<T, S> std::collections::HashSet<T, S>);
impl_via_len!(in<K, V> std::collections::BTreeMap<K, V>);
impl_via_len!(in<T> std::collections::BTreeSet<T>);
impl_via_len!(in<T> std::collections::VecDeque<T>);
impl_via_len!(in<T> std::collections::BinaryHeap<T>);
impl_via_len!(in<T> std::collections::LinkedList<T>);
impl_via_len!(in<'a, K, V, S> &'a std::collections::HashMap<K, V, S>);
impl_via_len!(in<'a, T, S> &'a std::collections::HashSet<T, S>);
impl_via_len!(in<'a, K, V> &'a std::collections::BTreeMap<K, V>);
impl_via_len!(in<'a, T> &'a std::collections::BTreeSet<T>);
impl_via_len!(in<'a, T> &'a std::collections::VecDeque<T>);
impl_via_len!(in<'a, T> &'a std::collections::BinaryHeap<T>);
impl_via_len!(in<'a, T> &'a std::collections::LinkedList<T>);
