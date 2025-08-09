use proc_macro_error2::abort;
use syn::spanned::Spanned;
use syn::{Attribute, ItemFn};

/// Represents the attributes that can be applied to a function in the test module
pub(crate) enum FuncAttribute {
    Init,
    Test,
    ShouldPanic,
    Ignore,
    Timeout(TimeoutAttribute),
}

impl FuncAttribute {
    /// Tries to convert a `syn::Attribute` into a `FuncAttribute`.
    /// If the attribute is recognized, it returns `Some(FuncAttribute)`, otherwise `None`.
    /// Aborts with a compiler error if the attribute is malformed.
    fn try_from_attr(attr: &Attribute) -> Option<Self> {
        let ident = attr.path().get_ident()?.to_string();
        Some(match ident.as_str() {
            "init" => FuncAttribute::Init,
            "test" => FuncAttribute::Test,
            "should_panic" => FuncAttribute::ShouldPanic,
            "ignore" => FuncAttribute::Ignore,
            "timeout" => FuncAttribute::Timeout(TimeoutAttribute::from_attr(attr)),
            _ => return None,
        })
    }
}

pub(crate) struct TimeoutAttribute {
    pub value: u32,
}

impl syn::parse::Parse for TimeoutAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value_lit: syn::LitInt = input.parse()?;
        let value = value_lit.base10_parse::<u32>()?;

        Ok(TimeoutAttribute { value })
    }
}

impl TimeoutAttribute {
    fn from_attr(attr: &Attribute) -> Self {
        match attr.parse_args::<TimeoutAttribute>() {
            Ok(timeout_attr) => timeout_attr,
            Err(e) => {
                abort!(
                    attr,
                    "failed to parse `timeout` attribute. Must be of the form #[timeout(10)] where 10 is the timeout in seconds. Error: {}",
                    e
                );
            }
        }
    }
}

pub(crate) struct FunctionWithAttributes {
    /// Original function item without the attributes that we recognize
    pub func: ItemFn,
    /// All recognized attributes that were attached to the function
    pub attributes: Vec<(FuncAttribute, proc_macro2::Span)>,
}

impl From<ItemFn> for FunctionWithAttributes {
    fn from(mut func: ItemFn) -> Self {
        let mut attributes = vec![];
        func.attrs.retain(|attr| {
            if let Some(func_attr) = FuncAttribute::try_from_attr(attr) {
                attributes.push((func_attr, attr.path().span()));
                false
            } else {
                true
            }
        });

        Self { func, attributes }
    }
}
