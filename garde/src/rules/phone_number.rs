//! Phone number validation using the [`phonenumber`] crate.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(phone_number)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`PhoneNumber`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(phone_number)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use std::fmt::Display;
use std::str::FromStr;

use super::AsStr;
use crate::error::{Error, StandardErrorCode};

pub fn apply<T: PhoneNumber>(v: &T, _: ()) -> Result<(), Error> {
    match v.validate_phone_number() {
        Ok(true) => Ok(()),
        Ok(false) => Err(Error::with_code(
            "not a valid phone number",
            StandardErrorCode::PhoneNumberInvalid,
        )),
        Err(e) => Err(Error::with_code(
            format!("not a valid phone number: {e}"),
            StandardErrorCode::PhoneNumberInvalid,
        )),
    }
}

pub fn apply_with_code<T: PhoneNumber>(v: &T, _: (), code: &str) -> Result<(), Error> {
    match v.validate_phone_number() {
        Ok(true) => Ok(()),
        Ok(false) => Err(Error::with_custom_code(
            "not a valid phone number",
            code,
        )),
        Err(e) => Err(Error::with_custom_code(
            format!("not a valid phone number: {e}"),
            code,
        )),
    }
}

pub trait PhoneNumber {
    type Error: Display;

    fn validate_phone_number(&self) -> Result<bool, Self::Error>;
}

impl<T: AsStr> PhoneNumber for T {
    type Error = phonenumber::ParseError;

    fn validate_phone_number(&self) -> Result<bool, Self::Error> {
        let number = phonenumber::PhoneNumber::from_str(self.as_str())?;
        Ok(number.is_valid())
    }
}

impl<T: PhoneNumber> PhoneNumber for Option<T> {
    type Error = T::Error;

    fn validate_phone_number(&self) -> Result<bool, Self::Error> {
        match self {
            Some(value) => value.validate_phone_number(),
            None => Ok(true),
        }
    }
}
