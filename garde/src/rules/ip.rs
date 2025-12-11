//! IP validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(ip)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Ip`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(ip)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use std::fmt::Display;

use super::AsStr;
// Re-export IpKind for use in generated code
pub use crate::error::IpKind;
use crate::error::{Error, ErrorKind};

pub fn apply<T: Ip>(v: &T, (kind,): (IpKind,)) -> Result<(), Error> {
    if v.validate_ip(kind).is_err() {
        return Err(Error::from_kind(ErrorKind::InvalidIp { expected: kind }));
    }
    Ok(())
}

pub trait Ip {
    type Error: Display;

    fn validate_ip(&self, kind: IpKind) -> Result<(), Self::Error>;
}

impl<T: AsStr> Ip for T {
    type Error = std::net::AddrParseError;

    fn validate_ip(&self, kind: IpKind) -> Result<(), Self::Error> {
        let v = self.as_str();
        match kind {
            IpKind::Any => {
                let _ = v.parse::<std::net::IpAddr>()?;
            }
            IpKind::V4 => {
                let _ = v.parse::<std::net::Ipv4Addr>()?;
            }
            IpKind::V6 => {
                let _ = v.parse::<std::net::Ipv6Addr>()?;
            }
        };
        Ok(())
    }
}

impl<T: Ip> Ip for Option<T> {
    type Error = T::Error;

    fn validate_ip(&self, kind: IpKind) -> Result<(), Self::Error> {
        match self {
            Some(value) => value.validate_ip(kind),
            None => Ok(()),
        }
    }
}
