use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};

use super::*;

impl ErrorKind {
    /// Format this error using a FluentBundle.
    ///
    /// Uses `self.code()` as the message id and `self.into_params()` as arguments.
    /// Falls back to `default_message()` if the message is not found.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use garde::error::ErrorKind;
    /// use fluent_bundle::{FluentBundle, FluentResource};
    /// use unic_langid::langid;
    ///
    /// let ftl = r#"
    /// length-too_short = Length must be at least { $min } (got { $actual })
    /// "#;
    ///
    /// let resource = FluentResource::try_new(ftl.to_string()).unwrap();
    /// let langid = langid!("en-US");
    /// let mut bundle = FluentBundle::new_concurrent(vec![langid]);
    /// bundle.add_resource(resource).unwrap();
    ///
    /// let error = ErrorKind::LengthTooShort {
    ///     min: garde::error::LengthBound::Inclusive(5),
    ///     actual: 3,
    /// };
    /// let message = error.format_with_bundle(&bundle);
    /// ```
    pub fn format_with_bundle(self, bundle: &FluentBundle<FluentResource>) -> String {
        let message_id = self.fluent_code();

        // Try to get the fluent message first (before consuming self)
        let maybe_pattern = bundle
            .get_message(&message_id)
            .and_then(|m| m.value().map(|p| p.clone()));

        let params = self.into_params();
        let args = params.to_fluent_args();

        if let Some(pattern) = maybe_pattern {
            let mut errors = vec![];
            let result = bundle.format_pattern(&pattern, Some(&args), &mut errors);
            if errors.is_empty() {
                return result.into_owned();
            }
        }

        // Fallback to default message if fluent formatting fails
        format!("validation failed: {}", message_id)
    }
}

impl Error {
    /// Format this error using a FluentBundle.
    ///
    /// Delegates to `ErrorKind::format_with_bundle`.
    pub fn format_with_bundle(self, bundle: &FluentBundle<FluentResource>) -> String {
        self.kind.format_with_bundle(bundle)
    }
}

impl Params {
    /// Convert Params to FluentArgs.
    pub fn to_fluent_args(&self) -> FluentArgs<'_> {
        let mut args = FluentArgs::new();
        for (key, value) in self.iter() {
            match value {
                ParamValue::String(s) => {
                    args.set(key.as_str(), s.as_str());
                }
                ParamValue::Integer(i) => {
                    args.set(key.as_str(), *i);
                }
                ParamValue::Unsigned(u) => {
                    args.set(key.as_str(), *u as i64);
                }
                ParamValue::Float(f) => {
                    args.set(key.as_str(), *f);
                }
                ParamValue::Bool(b) => {
                    args.set(key.as_str(), if *b { "true" } else { "false" });
                }
            }
        }
        args
    }
}
