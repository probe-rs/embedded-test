use crate::attributes::tests::parse::{FuncAttribute, FunctionWithAttributes};
use syn::{spanned::Spanned, Attribute, ItemFn, ReturnType, Type};

pub(crate) struct InitFunc {
    pub name: String,
    pub func: ItemFn,
    pub state: Option<Type>,
    pub asyncness: bool,
}

impl TryFrom<FunctionWithAttributes> for InitFunc {
    type Error = syn::Error;

    fn try_from(func: FunctionWithAttributes) -> syn::Result<Self> {
        let FunctionWithAttributes { func, attributes } = func;
        for (attr, span) in attributes {
            match attr {
                FuncAttribute::Init => {}
                FuncAttribute::Test(_) => unreachable!(),
                _ => {
                    return Err(syn::Error::new(
                        span,
                        "The `#[init]` function can not have this attribute",
                    ))
                }
            }
        }
        if check_fn_sig(&func.sig).is_err() || !func.sig.inputs.is_empty() {
            return Err(syn::Error::new(
                func.sig.span(),
                "`#[init]` function must have signature `async fn() [-> Type]` (async/return type are optional)",
            ));
        }

        if cfg!(not(feature = "embassy")) && func.sig.asyncness.is_some() {
            return Err(syn::Error::new(
                func.sig.span(),
                "`#[init]` function can only be async if an async executor is enabled via feature",
            ));
        }

        let state = match &func.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(.., ty) => Some(*ty.clone()),
        };
        Ok(InitFunc {
            name: func.sig.ident.to_string(),
            asyncness: func.sig.asyncness.is_some(),
            func,
            state,
        })
    }
}

pub(crate) struct TestFunc {
    pub func: ItemFn,
    pub cfgs: Vec<Attribute>,
    pub input: Option<Type>,
    pub should_panic: bool,
    pub ignore: bool,
    pub asyncness: bool,
    pub timeout: Option<u32>,
    pub custom_init: Option<syn::Ident>,
}

impl TryFrom<FunctionWithAttributes> for TestFunc {
    type Error = syn::Error;

    fn try_from(func: FunctionWithAttributes) -> syn::Result<Self> {
        let FunctionWithAttributes { func, attributes } = func;
        let mut should_panic = false;
        let mut ignore = false;
        let mut timeout = None;
        let mut custom_init = None;
        for (attr, _span) in attributes {
            match attr {
                FuncAttribute::Init => unreachable!(),
                FuncAttribute::Test(attr) => custom_init = attr.init,
                FuncAttribute::ShouldPanic => should_panic = true,
                FuncAttribute::Ignore => ignore = true,
                FuncAttribute::Timeout(t) => timeout = Some(t.value),
            }
        }

        if check_fn_sig(&func.sig).is_err() || func.sig.inputs.len() > 1 {
            return Err(syn::Error::new(
                func.sig.span(),
                "`#[test]` function must have signature `async fn(state: Type)` (async/parameter are optional)",
            ));
        }

        if cfg!(not(feature = "embassy")) && func.sig.asyncness.is_some() {
            return Err(syn::Error::new(
                func.sig.span(),
                "`#[test]` function can only be async if an async executor is enabled via feature",
            ));
        }

        let input = if func.sig.inputs.len() == 1 {
            Some(extract_single_value_arg(&func.sig.inputs[0])?)
            // NOTE we cannot check the argument type matches `init.state` at this point
        } else {
            None
        };

        Ok(TestFunc {
            cfgs: extract_cfgs(&func.attrs),
            asyncness: func.sig.asyncness.is_some(),
            func,
            input,
            should_panic,
            ignore,
            timeout,
            custom_init,
        })
    }
}

pub(crate) struct OtherFunc(pub FunctionWithAttributes);
impl TryFrom<FunctionWithAttributes> for OtherFunc {
    type Error = syn::Error;

    fn try_from(func: FunctionWithAttributes) -> syn::Result<Self> {
        if let Some((_attr, span)) = func.attributes.first() {
            return Err(syn::Error::new(
                *span,
                "Only `#[test]` or `#[init]` functions can have such an attribute",
            ));
        }
        Ok(OtherFunc(func))
    }
}

pub(crate) enum AnnotatedFunction {
    Init(InitFunc),
    Test(TestFunc),
    Other(OtherFunc),
}

impl TryFrom<FunctionWithAttributes> for AnnotatedFunction {
    type Error = syn::Error;

    fn try_from(func: FunctionWithAttributes) -> syn::Result<Self> {
        enum FuncKind {
            Init,
            Test,
        }
        let mut func_kind = None;
        for (attr, span) in &func.attributes {
            match attr {
                FuncAttribute::Init if func_kind.is_none() => func_kind = Some(FuncKind::Init),
                FuncAttribute::Test(_) if func_kind.is_none() => func_kind = Some(FuncKind::Test),
                FuncAttribute::Init | FuncAttribute::Test(_) => {
                    return Err(syn::Error::new(
                        *span,
                        "A function can only be marked with one of `#[init]` or `#[test]`",
                    ));
                }
                _ => {}
            }
        }

        Ok(match func_kind {
            Some(FuncKind::Init) => AnnotatedFunction::Init(InitFunc::try_from(func)?),
            Some(FuncKind::Test) => AnnotatedFunction::Test(TestFunc::try_from(func)?),
            None => AnnotatedFunction::Other(OtherFunc::try_from(func)?),
        })
    }
}

fn extract_cfgs(attrs: &[Attribute]) -> Vec<Attribute> {
    let mut cfgs = vec![];

    for attr in attrs {
        if attr.path().is_ident("cfg") {
            cfgs.push(attr.clone());
        }
    }

    cfgs
}

// NOTE doesn't check the parameters or the return type
fn check_fn_sig(sig: &syn::Signature) -> Result<(), ()> {
    if sig.constness.is_none()
        && !matches!(sig.safety, syn::Safety::Unsafe(_))
        && sig.abi.is_none()
        && sig.generics.params.is_empty()
        && sig.generics.where_clause.is_none()
        && sig.variadic.is_none()
    {
        Ok(())
    } else {
        Err(())
    }
}

fn extract_single_value_arg(arg: &syn::FnArg) -> syn::Result<Type> {
    if let syn::FnArg::Typed(pat) = arg {
        match &*pat.ty {
            syn::Type::Reference(_) => {}
            _ => return Ok(*pat.ty.clone()),
        }
    }
    Err(syn::Error::new(
        arg.span(),
        "parameter must be a single value, not a reference",
    ))
}
