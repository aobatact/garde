use std::collections::BTreeMap;

use proc_macro2::{Ident, Span};
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::As;
use syn::{DeriveInput, Expr, Token, Type};

use crate::model;
use crate::model::List;
use crate::util::{default_ctx_name, MaybeFoldError};

pub fn parse(input: DeriveInput) -> syn::Result<model::Input> {
    let mut error = None;

    let ident = input.ident.clone();
    let generics = input.generics.clone();
    let attrs = match parse_input_attr_list(&input.attrs) {
        Ok(v) => v,
        Err(e) => {
            error.maybe_fold(e);
            Vec::new()
        }
    };
    let kind = match &input.data {
        syn::Data::Struct(v) => parse_struct(v),
        syn::Data::Enum(v) => parse_enum(v),
        syn::Data::Union(v) => parse_union(v),
    };
    let kind = match kind {
        Ok(kind) => kind,
        Err(e) => {
            error.maybe_fold(e);
            model::InputKind::empty()
        }
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::Input {
        ident,
        generics,
        attrs,
        kind,
    })
}

fn parse_input_attr_list(attrs: &[syn::Attribute]) -> syn::Result<Vec<(Span, model::Attr)>> {
    let mut error = None;
    let mut out = Vec::new();

    for attr in attrs.iter() {
        if attr.path().is_ident("garde") {
            match parse_input_attr(attr) {
                Ok(v) => out.push((attr.span(), v)),
                Err(e) => error.maybe_fold(e),
            }
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(out)
}

fn parse_input_attr(attr: &syn::Attribute) -> syn::Result<model::Attr> {
    let meta_list = match attr.meta.require_list() {
        Ok(v) => v,
        Err(_) => {
            return Err(syn::Error::new(
                attr.meta.span(),
                "invalid attr style, expected parenthesized arguments",
            ))
        }
    };

    syn::parse2::<model::Attr>(meta_list.tokens.clone())
}

impl Parse for model::Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        match ident.to_string().as_str() {
            "context" => {
                let content;
                syn::parenthesized!(content in input);
                let ty = content.parse::<Type>()?;
                let ident = if content.parse::<As>().is_ok() {
                    content.parse()?
                } else {
                    default_ctx_name()
                };

                Ok(model::Attr::Context(Box::new(ty), ident))
            }
            "allow_unvalidated" => Ok(model::Attr::AllowUnvalidated),
            "transparent" => Ok(model::Attr::Transparent),
            _ => Err(syn::Error::new(ident.span(), "unrecognized attribute")),
        }
    }
}

fn parse_struct(node: &syn::DataStruct) -> syn::Result<model::InputKind> {
    let mut error = None;

    let fields = match parse_variant(&node.fields) {
        Ok(Some(v)) => v,
        Ok(None) => {
            error.maybe_fold(syn::Error::new(
                node.fields.span(),
                "unit structs are unsupported",
            ));
            model::Variant::empty()
        }
        Err(e) => {
            error.maybe_fold(e);
            model::Variant::empty()
        }
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::InputKind::Struct(fields))
}

fn parse_enum(node: &syn::DataEnum) -> syn::Result<model::InputKind> {
    let mut error = None;
    let mut variants = Vec::new();

    for variant in node.variants.iter() {
        match parse_variant(&variant.fields) {
            Ok(v) => variants.push((variant.ident.clone(), v)),
            Err(e) => error.maybe_fold(e),
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::InputKind::Enum(variants))
}

fn parse_union(node: &syn::DataUnion) -> syn::Result<model::InputKind> {
    Err(syn::Error::new(
        node.union_token.span(),
        "unions are unsupported",
    ))
}

fn parse_variant(fields: &syn::Fields) -> syn::Result<Option<model::Variant>> {
    let mut error = None;

    let variant = match fields {
        syn::Fields::Named(v) => {
            let mut fields = BTreeMap::new();
            for field in v.named.iter() {
                let ident = field.ident.clone().unwrap();
                let ty = field.ty.clone();
                let rules = match parse_field_attr_list(&field.attrs) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        Vec::new()
                    }
                };
                fields.insert(ident, model::Field { ty, rules });
            }
            Some(model::Variant::Struct(fields))
        }
        syn::Fields::Unnamed(v) => {
            let mut fields = Vec::new();
            for field in v.unnamed.iter() {
                let ty = field.ty.clone();
                let rules = match parse_field_attr_list(&field.attrs) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        Vec::new()
                    }
                };
                fields.push(model::Field { ty, rules });
            }
            Some(model::Variant::Tuple(fields))
        }
        syn::Fields::Unit => None,
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(variant)
}

fn parse_field_attr_list(attrs: &[syn::Attribute]) -> syn::Result<Vec<model::RawRule>> {
    let mut error = None;
    let mut rules = Vec::new();

    for attr in attrs.iter() {
        if attr.path().is_ident("garde") {
            match attr.parse_args_with(Punctuated::<_, syn::token::Comma>::parse_terminated) {
                Ok(list) => {
                    for rule in list {
                        match rule {
                            ContinueOnFail::Ok(v) => rules.push(v),
                            ContinueOnFail::Err(e) => error.maybe_fold(e),
                        }
                    }
                }
                Err(e) => error.maybe_fold(e),
            }
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(rules)
}

enum ContinueOnFail<T> {
    Ok(T),
    Err(syn::Error),
}

impl<T> Parse for ContinueOnFail<T>
where
    T: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match <T as Parse>::parse(input) {
            Ok(v) => Ok(ContinueOnFail::Ok(v)),
            Err(e) => Ok(ContinueOnFail::Err(e)),
        }
    }
}

/// Helper to parse optional `code = "..."` from a parse stream
fn parse_optional_code(input: syn::parse::ParseStream) -> syn::Result<Option<model::Str>> {
    if input.is_empty() {
        return Ok(None);
    }
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        if input.is_empty() {
            return Ok(None);
        }
        let ident = input.parse::<Ident>()?;
        if ident != "code" {
            return Err(syn::Error::new(ident.span(), "expected 'code'"));
        }
        input.parse::<Token![=]>()?;
        Ok(Some(model::Str::parse(input)?))
    } else {
        Ok(None)
    }
}

impl Parse for model::RawRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        let span = ident.span();

        // Helper macro for rules that don't support code parameter
        macro_rules! no_code_rule {
            ($rule:ident $(($content:expr))?) => {
                Ok(model::RawRule {
                    span,
                    kind: model::RawRuleKind::$rule$(($content))?,
                    code: None,
                })
            };
        }

        // Helper macro for simple rules (no args) that support optional code
        macro_rules! simple_rule {
            ($rule:ident) => {{
                let code = if input.peek(syn::token::Paren) {
                    let content;
                    syn::parenthesized!(content in input);
                    if content.is_empty() {
                        None
                    } else {
                        // Parse code = "..."
                        let ident = content.parse::<Ident>()?;
                        if ident != "code" {
                            return Err(syn::Error::new(ident.span(), "expected 'code'"));
                        }
                        content.parse::<Token![=]>()?;
                        Some(model::Str::parse(&content)?)
                    }
                } else {
                    None
                };
                Ok(model::RawRule {
                    span,
                    kind: model::RawRuleKind::$rule,
                    code,
                })
            }};
        }

        // Helper macro for rules with required content that support optional code
        macro_rules! content_rule {
            ($rule:ident, $content_ty:ty) => {{
                let content;
                syn::parenthesized!(content in input);
                let value = content.parse::<$content_ty>()?;
                let code = parse_optional_code(&content)?;
                Ok(model::RawRule {
                    span,
                    kind: model::RawRuleKind::$rule(value),
                    code,
                })
            }};
        }

        match ident.to_string().as_str() {
            // Rules that don't support code parameter
            "skip" => no_code_rule!(Skip),
            "adapt" => {
                let content;
                syn::parenthesized!(content in input);
                no_code_rule!(Adapt(content.parse()?))
            }
            "rename" => {
                let content;
                syn::parenthesized!(content in input);
                no_code_rule!(Rename(content.parse()?))
            }
            "code" => {
                let content;
                syn::parenthesized!(content in input);
                no_code_rule!(Code(content.parse()?))
            }
            "dive" => {
                let ctx = if input.peek(syn::token::Paren) {
                    let content;
                    syn::parenthesized!(content in input);
                    if content.is_empty() {
                        None
                    } else {
                        Some(content.parse()?)
                    }
                } else {
                    None
                };
                no_code_rule!(Dive(ctx))
            }
            "inner" => {
                let content;
                syn::parenthesized!(content in input);
                no_code_rule!(Inner(content.parse()?))
            }
            "if" => {
                let content;
                syn::parenthesized!(content in input);
                no_code_rule!(If(content.parse()?))
            }

            // Simple rules (no required args) that support optional code
            "required" => simple_rule!(Required),
            "ascii" => simple_rule!(Ascii),
            "alphanumeric" => simple_rule!(Alphanumeric),
            "email" => simple_rule!(Email),
            "url" => simple_rule!(Url),
            "ip" => simple_rule!(Ip),
            "ipv4" => simple_rule!(IpV4),
            "ipv6" => simple_rule!(IpV6),
            "credit_card" => simple_rule!(CreditCard),
            "phone_number" => simple_rule!(PhoneNumber),

            // Rules with required content that support optional code
            "length" => content_rule!(Length, model::RawLength),
            "matches" => content_rule!(Matches, syn::Path),
            "range" => content_rule!(Range, model::Range<syn::Expr>),
            "contains" => content_rule!(Contains, syn::Expr),
            "prefix" => content_rule!(Prefix, syn::Expr),
            "suffix" => content_rule!(Suffix, syn::Expr),
            "pattern" => content_rule!(Pattern, model::Pattern),
            "custom" => content_rule!(Custom, syn::Expr),

            _ => Err(syn::Error::new(ident.span(), "unrecognized validation rule")),
        }
    }
}

impl Parse for model::Pattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Lit) {
            Ok(Self::Lit(model::Str::parse(input)?))
        } else {
            Ok(Self::Expr(syn::Expr::parse(input)?))
        }
    }
}

impl Parse for model::Str {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(model::Str {
            span: input.span(),
            value: <syn::LitStr as Parse>::parse(input)?.value(),
        })
    }
}

// impl Parse for model::Message {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         if input.peek(syn::LitStr) {
//             Ok(Self::Fmt(model::Str::parse(input)?))
//         } else {
//             Ok(Self::Func(syn::Expr::parse(input)?))
//         }
//     }
// }

impl Parse for model::RawLength {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        // Check if it starts with a mode identifier
        let starts_with_mode = if input.peek(syn::Ident) {
            let ident = input.fork().parse::<syn::Ident>().unwrap();
            matches!(ident.to_string().as_str(), "simple" | "bytes" | "chars" | "graphemes" | "utf16")
        } else {
            false
        };

        if starts_with_mode {
            // Parse mode first
            let mode_ident = input.parse::<syn::Ident>()?;
            let mode = match mode_ident.to_string().as_str() {
                "simple" => model::LengthMode::Simple,
                "bytes" => model::LengthMode::Bytes,
                "chars" => model::LengthMode::Chars,
                "graphemes" => model::LengthMode::Graphemes,
                "utf16" => model::LengthMode::Utf16,
                _ => unreachable!(),
            };

            // Check if next token is a comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                
                // Parse as range expression
                let expr = input.parse::<syn::Expr>()?;
                Ok(model::RawLength {
                    mode,
                    range: model::Range {
                        span,
                        expr: model::Either::Right(expr),
                    },
                })
            } else {
                Err(syn::Error::new(
                    mode_ident.span(),
                    "length mode must be followed by a range expression (e.g., `bytes, 1..=10`)"
                ))
            }
        } else {
            // Parse as range expression with default mode
            let expr = input.parse::<syn::Expr>()?;
            Ok(model::RawLength {
                mode: model::LengthMode::default(),
                range: model::Range {
                    span,
                    expr: model::Either::Right(expr),
                },
            })
        }
    }
}


impl<T> Parse for model::Range<T>
where
    T: FromExpr,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let expr = input.parse::<syn::Expr>()?;
        
        Ok(model::Range {
            span,
            expr: T::from_expr(expr)?,
        })
    }
}

impl<T: Parse> Parse for List<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        type CommaSeparated<T> = Punctuated<T, Token![,]>;
        let contents: Vec<_> = CommaSeparated::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(Self { contents })
    }
}

impl Parse for model::IfRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse "cond = <expr>,"
        let cond_ident = input.parse::<Ident>()?;
        if cond_ident != "cond" {
            return Err(syn::Error::new(
                cond_ident.span(),
                "expected 'cond' in if rule",
            ));
        }
        input.parse::<Token![=]>()?;
        let condition = input.parse::<Expr>()?;

        // Expect comma after condition
        input.parse::<Token![,]>()?;

        // Parse the remaining rules
        let rules = input.parse::<List<model::RawRule>>()?;

        if rules.contents.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "if rule must contain at least one validation rule",
            ));
        }

        Ok(model::IfRule { condition, rules })
    }
}

trait FromExpr: Sized {
    fn from_expr(v: syn::Expr) -> syn::Result<Self>;
}

impl<L, R> FromExpr for model::Either<L, R>
where
    L: FromExpr,
    R: FromExpr,
{
    fn from_expr(v: syn::Expr) -> syn::Result<Self> {
        L::from_expr(v.clone())
            .map(model::Either::Left)
            .or_else(|_| R::from_expr(v).map(model::Either::Right))
    }
}

impl FromExpr for syn::Expr {
    fn from_expr(v: syn::Expr) -> syn::Result<Self> {
        Ok(v)
    }
}

impl FromExpr for usize {
    fn from_expr(v: syn::Expr) -> syn::Result<Self> {
        match v {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(v),
                ..
            }) => v.base10_parse(),
            _ => Err(syn::Error::new(v.span(), "expected usize")),
        }
    }
}
