//! Error types used by `garde`.
//!
//! The entrypoint of this module is the [`Error`] type.
#![allow(dead_code)]

#[cfg(feature = "fluent")]
mod fluent_support;

mod rc_list;
use std::borrow::Cow;
use std::collections::HashMap;

use compact_str::{format_compact, CompactString, ToCompactString};
#[cfg(feature = "fluent")]
pub use fluent_support::*;
use smallvec::SmallVec;

use self::rc_list::List;

// ============================================================================
// ErrorKind and related types
// ============================================================================

/// Represents the kind of error that occurred during validation.
/// Internal rules use specific variants with typed parameters.
/// Custom rules use the `Custom` variant with a string code.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[non_exhaustive]
pub enum ErrorKind {
    // Length-related errors
    LengthTooShort {
        min: LengthBound,
        actual: usize,
    },
    LengthTooLong {
        max: LengthBound,
        actual: usize,
    },

    // Range-related errors
    RangeTooLow {
        min: RangeBound,
        actual: CompactString,
    },
    RangeTooHigh {
        max: RangeBound,
        actual: CompactString,
    },

    // Format validations
    InvalidEmail {
        reason: EmailErrorReason,
    },
    InvalidUrl {
        reason: CompactString,
    },
    InvalidIp {
        expected: IpKind,
    },
    InvalidCreditCard {
        reason: CompactString,
    },
    InvalidPhoneNumber {
        reason: CompactString,
    },

    // Character validations
    NotAscii,
    NotAlphanumeric,

    // String content validations
    PatternMismatch {
        pattern: CompactString,
    },
    MissingSubstring {
        expected: CompactString,
    },
    MissingPrefix {
        expected: CompactString,
    },
    MissingSuffix {
        expected: CompactString,
    },

    // Field validations
    FieldMismatch {
        other_field: CompactString,
    },
    Required,

    // Custom error codes (for user-defined rules or code overrides)
    Custom {
        code: CompactString,
        #[cfg_attr(feature = "serde", serde(flatten))]
        params: Params,
    },
}

impl ErrorKind {
    /// Get the error code string for this error kind (useful for i18n key lookup).
    pub fn code(&self) -> Cow<'_, str> {
        match self {
            ErrorKind::LengthTooShort { .. } => Cow::Borrowed("length.too_short"),
            ErrorKind::LengthTooLong { .. } => Cow::Borrowed("length.too_long"),
            ErrorKind::RangeTooLow { .. } => Cow::Borrowed("range.too_low"),
            ErrorKind::RangeTooHigh { .. } => Cow::Borrowed("range.too_high"),
            ErrorKind::InvalidEmail { .. } => Cow::Borrowed("email.invalid"),
            ErrorKind::InvalidUrl { .. } => Cow::Borrowed("url.invalid"),
            ErrorKind::InvalidIp { expected } => match expected {
                IpKind::Any => Cow::Borrowed("ip.invalid"),
                IpKind::V4 => Cow::Borrowed("ip.invalid_v4"),
                IpKind::V6 => Cow::Borrowed("ip.invalid_v6"),
            },
            ErrorKind::InvalidCreditCard { .. } => Cow::Borrowed("credit_card.invalid"),
            ErrorKind::InvalidPhoneNumber { .. } => Cow::Borrowed("phone_number.invalid"),
            ErrorKind::NotAscii => Cow::Borrowed("ascii.invalid"),
            ErrorKind::NotAlphanumeric => Cow::Borrowed("alphanumeric.invalid"),
            ErrorKind::PatternMismatch { .. } => Cow::Borrowed("pattern.mismatch"),
            ErrorKind::MissingSubstring { .. } => Cow::Borrowed("contains.missing"),
            ErrorKind::MissingPrefix { .. } => Cow::Borrowed("prefix.missing"),
            ErrorKind::MissingSuffix { .. } => Cow::Borrowed("suffix.missing"),
            ErrorKind::FieldMismatch { .. } => Cow::Borrowed("matches.mismatch"),
            ErrorKind::Required => Cow::Borrowed("required"),
            ErrorKind::Custom { code, .. } => Cow::Borrowed(code.as_str()),
        }
    }

    /// Get the fluent message id for this error kind.
    /// Uses hyphens instead of dots (e.g., "length-too_short" instead of "length.too_short").
    /// For Custom variants, this allocates a new string.
    pub fn fluent_code(&self) -> Cow<'static, str> {
        match self {
            ErrorKind::LengthTooShort { .. } => Cow::Borrowed("length-too_short"),
            ErrorKind::LengthTooLong { .. } => Cow::Borrowed("length-too_long"),
            ErrorKind::RangeTooLow { .. } => Cow::Borrowed("range-too_low"),
            ErrorKind::RangeTooHigh { .. } => Cow::Borrowed("range-too_high"),
            ErrorKind::InvalidEmail { .. } => Cow::Borrowed("email-invalid"),
            ErrorKind::InvalidUrl { .. } => Cow::Borrowed("url-invalid"),
            ErrorKind::InvalidIp { expected } => match expected {
                IpKind::Any => Cow::Borrowed("ip-invalid"),
                IpKind::V4 => Cow::Borrowed("ip-invalid_v4"),
                IpKind::V6 => Cow::Borrowed("ip-invalid_v6"),
            },
            ErrorKind::InvalidCreditCard { .. } => Cow::Borrowed("credit_card-invalid"),
            ErrorKind::InvalidPhoneNumber { .. } => Cow::Borrowed("phone_number-invalid"),
            ErrorKind::NotAscii => Cow::Borrowed("ascii-invalid"),
            ErrorKind::NotAlphanumeric => Cow::Borrowed("alphanumeric-invalid"),
            ErrorKind::PatternMismatch { .. } => Cow::Borrowed("pattern-mismatch"),
            ErrorKind::MissingSubstring { .. } => Cow::Borrowed("contains-missing"),
            ErrorKind::MissingPrefix { .. } => Cow::Borrowed("prefix-missing"),
            ErrorKind::MissingSuffix { .. } => Cow::Borrowed("suffix-missing"),
            ErrorKind::FieldMismatch { .. } => Cow::Borrowed("matches-mismatch"),
            ErrorKind::Required => Cow::Borrowed("required"),
            ErrorKind::Custom { code, .. } => Cow::Owned(code.replace('.', "-")),
        }
    }

    /// Generate a default English message for this error kind.
    pub fn default_message(&self) -> CompactString {
        match self {
            ErrorKind::LengthTooShort { min, .. } => {
                format_compact!("length is lower than {}", min.value())
            }
            ErrorKind::LengthTooLong { max, .. } => {
                format_compact!("length is greater than {}", max.value())
            }
            ErrorKind::RangeTooLow { min, .. } => {
                format_compact!("lower than {}", min.value_str())
            }
            ErrorKind::RangeTooHigh { max, .. } => {
                format_compact!("greater than {}", max.value_str())
            }
            ErrorKind::InvalidEmail { reason } => {
                format_compact!("not a valid email: {}", reason)
            }
            ErrorKind::InvalidUrl { reason } => {
                format_compact!("not a valid url: {}", reason)
            }
            ErrorKind::InvalidIp { expected } => match expected {
                IpKind::Any => CompactString::const_new("not a valid IP address"),
                IpKind::V4 => CompactString::const_new("not a valid IPv4 address"),
                IpKind::V6 => CompactString::const_new("not a valid IPv6 address"),
            },
            ErrorKind::InvalidCreditCard { reason } => {
                format_compact!("not a valid credit card number: {}", reason)
            }
            ErrorKind::InvalidPhoneNumber { reason } => {
                format_compact!("not a valid phone number: {}", reason)
            }
            ErrorKind::NotAscii => CompactString::const_new("not ascii"),
            ErrorKind::NotAlphanumeric => CompactString::const_new("not alphanumeric"),
            ErrorKind::PatternMismatch { pattern } => {
                format_compact!("does not match pattern /{}/", pattern)
            }
            ErrorKind::MissingSubstring { expected } => {
                format_compact!("does not contain \"{}\"", expected)
            }
            ErrorKind::MissingPrefix { expected } => {
                format_compact!("value does not begin with \"{}\"", expected)
            }
            ErrorKind::MissingSuffix { expected } => {
                format_compact!("does not end with \"{}\"", expected)
            }
            ErrorKind::FieldMismatch { other_field } => {
                format_compact!("does not match {} field", other_field)
            }
            ErrorKind::Required => CompactString::const_new("not set"),
            ErrorKind::Custom { code, .. } => {
                format_compact!("validation failed: {}", code)
            }
        }
    }

    /// Extract parameters from this error kind as a Params struct.
    /// Useful for i18n template interpolation.
    pub fn into_params(self) -> Params {
        let mut params = Params::new();
        match self {
            ErrorKind::LengthTooShort { min, actual } => {
                params.insert(CompactString::const_new("min"), min.value());
                params.insert(CompactString::const_new("actual"), actual);
            }
            ErrorKind::LengthTooLong { max, actual } => {
                params.insert(CompactString::const_new("max"), max.value());
                params.insert(CompactString::const_new("actual"), actual);
            }
            ErrorKind::RangeTooLow { min, actual } => {
                params.insert(CompactString::const_new("min"), min.value_str());
                params.insert(CompactString::const_new("actual"), actual);
            }
            ErrorKind::RangeTooHigh { max, actual } => {
                params.insert(CompactString::const_new("max"), max.value_str());
                params.insert(CompactString::const_new("actual"), actual);
            }
            ErrorKind::InvalidEmail { reason } => {
                params.insert(CompactString::const_new("reason"), reason.to_string());
            }
            ErrorKind::InvalidUrl { reason } => {
                params.insert(CompactString::const_new("reason"), reason);
            }
            ErrorKind::InvalidIp { expected } => {
                params.insert(CompactString::const_new("expected"), expected.to_string());
            }
            ErrorKind::InvalidCreditCard { reason } => {
                params.insert(CompactString::const_new("reason"), reason);
            }
            ErrorKind::InvalidPhoneNumber { reason } => {
                params.insert(CompactString::const_new("reason"), reason);
            }
            ErrorKind::NotAscii => {}
            ErrorKind::NotAlphanumeric => {}
            ErrorKind::PatternMismatch { pattern } => {
                params.insert(CompactString::const_new("pattern"), pattern);
            }
            ErrorKind::MissingSubstring { expected } => {
                params.insert(CompactString::const_new("expected"), expected);
            }
            ErrorKind::MissingPrefix { expected } => {
                params.insert(CompactString::const_new("expected"), expected);
            }
            ErrorKind::MissingSuffix { expected } => {
                params.insert(CompactString::const_new("expected"), expected);
            }
            ErrorKind::FieldMismatch { other_field } => {
                params.insert(CompactString::const_new("other_field"), other_field);
            }
            ErrorKind::Required => {}
            ErrorKind::Custom { params: p, .. } => {
                return p;
            }
        }
        params
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.default_message())
    }
}

// ============================================================================
// LengthBound
// ============================================================================

/// Represents a bound in a length constraint.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LengthBound {
    Inclusive(usize),
    Exclusive(usize),
}

impl LengthBound {
    /// Get the bound value.
    pub fn value(&self) -> usize {
        match self {
            LengthBound::Inclusive(v) => *v,
            LengthBound::Exclusive(v) => *v,
        }
    }

    /// Check if the bound is inclusive.
    pub fn is_inclusive(&self) -> bool {
        matches!(self, LengthBound::Inclusive(_))
    }
}

impl std::fmt::Display for LengthBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthBound::Inclusive(v) => write!(f, "{}", v),
            LengthBound::Exclusive(v) => write!(f, "{} (exclusive)", v),
        }
    }
}

// ============================================================================
// RangeBound
// ============================================================================

/// Represents a bound in a range constraint.
/// Uses string representation for flexibility with different numeric types.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum RangeBound {
    Inclusive(CompactString),
    Exclusive(CompactString),
}

impl RangeBound {
    /// Get the bound value as a string.
    pub fn value_str(&self) -> &str {
        match self {
            RangeBound::Inclusive(v) => v.as_str(),
            RangeBound::Exclusive(v) => v.as_str(),
        }
    }

    /// Check if the bound is inclusive.
    pub fn is_inclusive(&self) -> bool {
        matches!(self, RangeBound::Inclusive(_))
    }
}

impl std::fmt::Display for RangeBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeBound::Inclusive(v) => write!(f, "{}", v),
            RangeBound::Exclusive(v) => write!(f, "{} (exclusive)", v),
        }
    }
}

// ============================================================================
// EmailErrorReason
// ============================================================================

/// Detailed reason for email validation failure.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum EmailErrorReason {
    Empty,
    MissingAt,
    UserLengthExceeded,
    InvalidUser,
    DomainLengthExceeded,
    InvalidDomain,
}

impl std::fmt::Display for EmailErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailErrorReason::Empty => write!(f, "empty"),
            EmailErrorReason::MissingAt => write!(f, "missing `@`"),
            EmailErrorReason::UserLengthExceeded => write!(f, "user length exceeded"),
            EmailErrorReason::InvalidUser => write!(f, "invalid user"),
            EmailErrorReason::DomainLengthExceeded => write!(f, "domain length exceeded"),
            EmailErrorReason::InvalidDomain => write!(f, "invalid domain"),
        }
    }
}

// ============================================================================
// IpKind
// ============================================================================

/// IP address kind for validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum IpKind {
    Any,
    V4,
    V6,
}

impl std::fmt::Display for IpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpKind::Any => write!(f, "IP"),
            IpKind::V4 => write!(f, "IPv4"),
            IpKind::V6 => write!(f, "IPv6"),
        }
    }
}

// ============================================================================
// Params and ParamValue
// ============================================================================

/// Dynamic parameters for custom error codes.
/// Used for i18n template interpolation.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Params {
    #[cfg_attr(feature = "serde", serde(flatten))]
    inner: HashMap<CompactString, ParamValue>,
}

impl Params {
    /// Create an empty Params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a parameter.
    pub fn insert(&mut self, key: impl Into<CompactString>, value: impl Into<ParamValue>) {
        self.inner.insert(key.into(), value.into());
    }

    /// Get a parameter value.
    pub fn get(&self, key: &str) -> Option<&ParamValue> {
        self.inner.get(key)
    }

    /// Iterate over all parameters.
    pub fn iter(&self) -> impl Iterator<Item = (&CompactString, &ParamValue)> {
        self.inner.iter()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// A parameter value that can be serialized.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ParamValue {
    String(CompactString),
    Integer(i64),
    Unsigned(u64),
    Float(f64),
    Bool(bool),
}

impl std::fmt::Display for ParamValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamValue::String(v) => write!(f, "{}", v),
            ParamValue::Integer(v) => write!(f, "{}", v),
            ParamValue::Unsigned(v) => write!(f, "{}", v),
            ParamValue::Float(v) => write!(f, "{}", v),
            ParamValue::Bool(v) => write!(f, "{}", v),
        }
    }
}

// From implementations for ParamValue
impl From<String> for ParamValue {
    fn from(s: String) -> Self {
        ParamValue::String(s.into())
    }
}

impl From<&str> for ParamValue {
    fn from(s: &str) -> Self {
        ParamValue::String(s.into())
    }
}

impl From<CompactString> for ParamValue {
    fn from(s: CompactString) -> Self {
        ParamValue::String(s)
    }
}

impl From<i64> for ParamValue {
    fn from(v: i64) -> Self {
        ParamValue::Integer(v)
    }
}

impl From<i32> for ParamValue {
    fn from(v: i32) -> Self {
        ParamValue::Integer(v as i64)
    }
}

impl From<u64> for ParamValue {
    fn from(v: u64) -> Self {
        ParamValue::Unsigned(v)
    }
}

impl From<usize> for ParamValue {
    fn from(v: usize) -> Self {
        ParamValue::Unsigned(v as u64)
    }
}

impl From<f64> for ParamValue {
    fn from(v: f64) -> Self {
        ParamValue::Float(v)
    }
}

impl From<f32> for ParamValue {
    fn from(v: f32) -> Self {
        ParamValue::Float(v as f64)
    }
}

impl From<bool> for ParamValue {
    fn from(v: bool) -> Self {
        ParamValue::Bool(v)
    }
}

// ============================================================================
// Error
// ============================================================================

/// A validation error with structured code and optional message override.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Error {
    /// The structured error kind with typed parameters.
    pub kind: ErrorKind,
    /// Optional human-readable message override.
    /// If None, a default message is generated from the kind.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub message: Option<CompactString>,
}

impl Error {
    /// Create an error from an ErrorKind.
    pub fn from_kind(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }

    /// Create an error with a custom message override.
    pub fn with_message(kind: ErrorKind, message: impl Into<CompactString>) -> Self {
        Self {
            kind,
            message: Some(message.into()),
        }
    }

    /// Create a custom error with a string code.
    pub fn custom(code: impl Into<CompactString>) -> Self {
        Self::from_kind(ErrorKind::Custom {
            code: code.into(),
            params: Params::default(),
        })
    }

    /// Create a custom error with code and parameters.
    pub fn custom_with_params(code: impl Into<CompactString>, params: Params) -> Self {
        Self::from_kind(ErrorKind::Custom {
            code: code.into(),
            params,
        })
    }

    /// Get the error code as a string (for i18n key lookup).
    pub fn code(&self) -> Cow<'_, str> {
        self.kind.code()
    }

    /// Get the display message (either custom or generated).
    pub fn display_message(&self) -> Cow<'_, str> {
        if let Some(msg) = &self.message {
            return Cow::Borrowed(msg.as_str());
        }
        Cow::Owned(self.kind.default_message().into_string())
    }

    /// Override the error code with a custom code.
    /// Converts the error to a Custom variant, preserving parameters.
    pub fn with_custom_code(self, code: impl Into<CompactString>) -> Self {
        let params = self.kind.into_params();
        Self {
            kind: ErrorKind::Custom {
                code: code.into(),
                params,
            },
            message: self.message,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_message())
    }
}

impl std::error::Error for Error {}

// ============================================================================
// Report
// ============================================================================

/// A validation error report.
///
/// This type is used as a container for errors aggregated during validation.
/// It is a flat list of `(Path, Error)`.
/// A single field or list item may have any number of errors attached to it.
///
/// It is possible to extract all errors for specific field using the [`select`][`crate::select`] macro.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Report {
    errors: Vec<(Path, Error)>,
}

impl Report {
    /// Create an empty [`Report`].
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Append an [`Error`] into this report at the given [`Path`].
    pub fn append(&mut self, path: Path, error: Error) {
        self.errors.push((path, error));
    }

    /// Iterate over all `(Path, Error)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = &(Path, Error)> {
        self.errors.iter()
    }

    /// Returns `true` if the report contains no validation errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Converts into the inner validation errors.
    pub fn into_inner(self) -> Vec<(Path, Error)> {
        self.errors
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (path, error) in self.iter() {
            if path.is_empty() {
                writeln!(f, "{error}")?;
            } else {
                writeln!(f, "{path}: {error}")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for Report {}

// ============================================================================
// Path and related types
// ============================================================================

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    components: List<(Kind, CompactString)>,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Kind {
    None,
    Key,
    Index,
}

/// Represents a path component without a key. This is useful when the container
/// only ever holds a single key, which is the case for any 1-tuple struct.
///
/// For an example usage, see the implementation of [Inner][`crate::rules::inner::Inner`] for `Option`.
#[derive(Default)]
pub struct NoKey(());

impl std::fmt::Display for NoKey {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

pub trait PathComponentKind: std::fmt::Display + ToCompactString {
    fn component_kind() -> Kind;
}

macro_rules! impl_path_component_kind {
    ($(@$($G:lifetime)*;)? $T:ty => $which:ident) => {
        impl $(<$($G),*>)? PathComponentKind for $T {
            fn component_kind() -> Kind {
                Kind::$which
            }
        }
    }
}

impl_path_component_kind!(usize => Index);
impl_path_component_kind!(@'a; &'a str => Key);
impl_path_component_kind!(@'a; Cow<'a, str> => Key);
impl_path_component_kind!(String => Key);
impl_path_component_kind!(CompactString => Key);
impl_path_component_kind!(NoKey => None);

impl<T: PathComponentKind> PathComponentKind for &T {
    fn component_kind() -> Kind {
        T::component_kind()
    }
}

impl Path {
    pub fn empty() -> Self {
        Self {
            components: List::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn new<C: PathComponentKind>(component: C) -> Self {
        Self {
            components: List::new().append((C::component_kind(), component.to_compact_string())),
        }
    }

    pub fn join<C: PathComponentKind>(&self, component: C) -> Self {
        Self {
            components: self
                .components
                .append((C::component_kind(), component.to_compact_string())),
        }
    }

    #[doc(hidden)]
    pub fn __iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Kind, &CompactString)> + ExactSizeIterator {
        let mut components = TempComponents::with_capacity(self.components.len());
        for (kind, component) in self.components.iter() {
            components.push((*kind, component));
        }
        components.into_iter()
    }
}

type TempComponents<'a> = SmallVec<[(Kind, &'a CompactString); 8]>;

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct Components<'a> {
            path: &'a Path,
        }

        impl std::fmt::Debug for Components<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut list = f.debug_list();
                list.entries(self.path.__iter().rev().map(|(_, c)| c))
                    .finish()
            }
        }

        f.debug_struct("Path")
            .field("components", &Components { path: self })
            .finish()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut components = self.__iter().rev().peekable();
        let mut first = true;
        while let Some((kind, component)) = components.next() {
            if first && kind == Kind::Index {
                f.write_str("[")?;
            }
            first = false;
            f.write_str(component.as_str())?;
            if kind == Kind::Index {
                f.write_str("]")?;
            }
            if let Some((kind, _)) = components.peek() {
                match kind {
                    Kind::None => {}
                    Kind::Key => f.write_str(".")?,
                    Kind::Index => f.write_str("[")?,
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq as _;

        let components = self.__iter().rev();
        let mut seq = serializer.serialize_seq(Some(components.len()))?;
        for component in components {
            seq.serialize_element(&component)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut components = List::new();
        for v in SmallVec::<[(Kind, CompactString); 8]>::deserialize(deserializer)? {
            components = components.append(v);
        }
        Ok(Path { components })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _: () = {
        fn assert<T: Send>() {}
        let _ = assert::<Report>;
    };

    #[test]
    fn path_join() {
        let path = Path::new("a").join("b").join("c");
        assert_eq!(path.to_string(), "a.b.c");
    }

    #[test]
    fn error_kind_code() {
        let err = Error::from_kind(ErrorKind::LengthTooShort {
            min: LengthBound::Inclusive(5),
            actual: 3,
        });
        assert_eq!(err.code(), "length.too_short");

        let err = Error::custom("my.custom.code");
        assert_eq!(err.code(), "my.custom.code");
    }

    #[test]
    fn error_with_custom_code() {
        let err = Error::from_kind(ErrorKind::LengthTooShort {
            min: LengthBound::Inclusive(5),
            actual: 3,
        });
        let err = err.with_custom_code("password.too_short");

        assert_eq!(err.code(), "password.too_short");

        // Parameters should be preserved
        if let ErrorKind::Custom { params, .. } = &err.kind {
            assert_eq!(params.get("min"), Some(&ParamValue::Unsigned(5)));
            assert_eq!(params.get("actual"), Some(&ParamValue::Unsigned(3)));
        } else {
            panic!("Expected Custom variant");
        }
    }

    #[test]
    fn report_select() {
        let mut report = Report::new();
        report.append(
            Path::new("a").join("b"),
            Error::from_kind(ErrorKind::Required),
        );
        report.append(
            Path::new("a").join("b").join("c"),
            Error::from_kind(ErrorKind::NotAscii),
        );
        report.append(
            Path::new("a").join("b").join("c"),
            Error::from_kind(ErrorKind::NotAlphanumeric),
        );
        report.append(
            Path::new("array").join("0").join("c"),
            Error::from_kind(ErrorKind::NotAlphanumeric),
        );

        let errors: Vec<_> = crate::select!(report, a.b.c).collect();
        assert_eq!(errors.len(), 2);

        let errors: Vec<_> = crate::select!(report, array[0].c).collect();
        assert_eq!(errors.len(), 1);
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn roundtrip_serde() {
            let mut report = Report::new();
            report.append(
                Path::new("a").join(0),
                Error::from_kind(ErrorKind::LengthTooShort {
                    min: LengthBound::Inclusive(5),
                    actual: 3,
                }),
            );
            report.append(
                Path::new("a").join(1),
                Error::from_kind(ErrorKind::InvalidEmail {
                    reason: EmailErrorReason::MissingAt,
                }),
            );
            report.append(
                Path::new("b").join("c"),
                Error::from_kind(ErrorKind::Required),
            );

            let json = serde_json::to_string(&report).unwrap();
            let de: Report = serde_json::from_str(&json).unwrap();

            assert_eq!(report.errors.len(), de.errors.len());
        }
    }
}
