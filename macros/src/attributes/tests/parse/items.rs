use syn::spanned::Spanned;
use syn::{parse, Attribute, ItemFn, ReturnType, Type};

pub(crate) struct InitFunc {
    pub func: ItemFn,
    pub state: Option<Type>,
    pub asyncness: bool,
}

pub(crate) struct TestFunc {
    pub func: ItemFn,
    pub cfgs: Vec<Attribute>,
    pub input: Option<Type>,
    pub should_panic: bool,
    pub ignore: bool,
    pub asyncness: bool,
    pub timeout: Option<u32>,
}

pub(crate) enum Func {
    Init(InitFunc),
    Test(TestFunc),
}

pub(crate) enum FuncKind {
    Init,
    Test,
}

pub(crate) fn parse_item(mut f: ItemFn) -> Result<Func, syn::Error> {
    let mut func_kind = None;
    let mut should_panic = false;
    let mut ignore = false;
    let mut timeout = None;

    f.attrs.retain(|attr| {
        if attr.path().is_ident("init") {
            func_kind = Some(FuncKind::Init);
            false
        } else if attr.path().is_ident("test") {
            func_kind = Some(FuncKind::Test);
            false
        } else if attr.path().is_ident("should_panic") {
            should_panic = true;
            false
        } else if attr.path().is_ident("ignore") {
            ignore = true;
            false
        } else if attr.path().is_ident("timeout") {
            timeout = Some(attr.clone());
            false
        } else {
            true
        }
    });

    let timeout = if let Some(attr) = timeout {
        match attr.parse_args::<TimeoutAttribute>() {
            Ok(TimeoutAttribute { value }) => Some(value),
            Err(e) => {
                return Err(syn::Error::new(
                    attr.span(),
                    format!("failed to parse `timeout` attribute. Must be of the form #[timeout(10)] where 10 is the timeout in seconds. Error: {}", e)
                ));
            }
        }
    } else {
        None
    };

    let func_kind = match func_kind {
        Some(it) => it,
        None => {
            return Err(syn::Error::new(
                f.span(),
                "function requires `#[init]` or `#[test]` attribute",
            ));
        }
    };

    match func_kind {
        FuncKind::Init => {
            if should_panic {
                return Err(parse::Error::new(
                    f.sig.ident.span(),
                    "`#[should_panic]` is not allowed on the `#[init]` function",
                ));
            }

            if ignore {
                return Err(parse::Error::new(
                    f.sig.ident.span(),
                    "`#[ignore]` is not allowed on the `#[init]` function",
                ));
            }

            if timeout.is_some() {
                return Err(parse::Error::new(
                    f.sig.ident.span(),
                    "`#[timeout]` is not allowed on the `#[init]` function",
                ));
            }

            if check_fn_sig(&f.sig).is_err() || !f.sig.inputs.is_empty() {
                return Err(parse::Error::new(
                    f.sig.ident.span(),
                    "`#[init]` function must have signature `async fn() [-> Type]` (async/return type are optional)",
                ));
            }

            #[cfg(not(feature = "embassy"))]
            if f.sig.asyncness.is_some() {
                return Err(parse::Error::new(
                    f.sig.ident.span(),
                    "`#[init]` function can only be async if an async executor is enabled via feature",
                ));
            }

            let state = match &f.sig.output {
                ReturnType::Default => None,
                ReturnType::Type(.., ty) => Some(*ty.clone()),
            };
            let asyncness = f.sig.asyncness.is_some();
            Ok(Func::Init(InitFunc {
                func: f,
                state,
                asyncness,
            }))
        }

        FuncKind::Test => {
            if check_fn_sig(&f.sig).is_err() || f.sig.inputs.len() > 1 {
                return Err(parse::Error::new(
                    f.sig.ident.span(),
                    "`#[test]` function must have signature `async fn(state: &mut Type)` (async/parameter are optional)",
                ));
            }

            #[cfg(not(feature = "embassy"))]
            if f.sig.asyncness.is_some() {
                return Err(parse::Error::new(
                    f.sig.ident.span(),
                    "`#[test]` function can only be async if an async executor is enabled via feature",
                ));
            }

            let input = if f.sig.inputs.len() == 1 {
                let ty = extract_single_value_arg(&f.sig.inputs[0]);
                Some(ty.ok_or_else(|| {
                    parse::Error::new(
                        f.sig.inputs[0].span(),
                        "parameter must be a single value, not a reference",
                    )
                })?)
                // NOTE we cannot check the argument type matches `init.state` at this point
            } else {
                None
            };

            let asyncness = f.sig.asyncness.is_some();
            Ok(Func::Test(TestFunc {
                cfgs: extract_cfgs(&f.attrs),
                func: f,
                asyncness,
                input,
                should_panic,
                ignore,
                timeout,
            }))
        }
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

struct TimeoutAttribute {
    value: u32,
}

impl syn::parse::Parse for TimeoutAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value_lit: syn::LitInt = input.parse()?;
        let value = value_lit.base10_parse::<u32>()?;

        Ok(TimeoutAttribute { value })
    }
}

// NOTE doesn't check the parameters or the return type
fn check_fn_sig(sig: &syn::Signature) -> Result<(), ()> {
    if sig.constness.is_none()
        && sig.unsafety.is_none()
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

fn extract_single_value_arg(arg: &syn::FnArg) -> Option<Type> {
    if let syn::FnArg::Typed(pat) = arg {
        match &*pat.ty {
            syn::Type::Reference(_) => None,
            _ => Some(*pat.ty.clone()),
        }
    } else {
        None
    }
}
