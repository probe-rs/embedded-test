use crate::attributes::tests::parse::{FuncAttribute, FunctionWithAttributes};
use proc_macro_error2::abort;
use syn::{Attribute, ItemFn, ReturnType, Type};

pub(crate) struct InitFunc {
    pub func: ItemFn,
    pub state: Option<Type>,
    pub asyncness: bool,
}

impl From<FunctionWithAttributes> for InitFunc {
    fn from(func: FunctionWithAttributes) -> Self {
        let FunctionWithAttributes { func, attributes } = func;
        for (attr, span) in attributes {
            match attr {
                FuncAttribute::Init => {}
                FuncAttribute::Test => unreachable!(),
                _ => abort!(span, "The `#[init]` function can not have this attribute"),
            }
        }
        if check_fn_sig(&func.sig).is_err() || !func.sig.inputs.is_empty() {
            abort!(
                    func.sig,
                    "`#[init]` function must have signature `async fn() [-> Type]` (async/return type are optional)",
                );
        }

        if cfg!(not(feature = "embassy")) && func.sig.asyncness.is_some() {
            abort!(
                func.sig,
                "`#[init]` function can only be async if an async executor is enabled via feature",
            );
        }

        let state = match &func.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(.., ty) => Some(*ty.clone()),
        };
        InitFunc {
            asyncness: func.sig.asyncness.is_some(),
            func,
            state,
        }
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
}

impl From<FunctionWithAttributes> for TestFunc {
    fn from(func: FunctionWithAttributes) -> Self {
        let FunctionWithAttributes { func, attributes } = func;
        let mut should_panic = false;
        let mut ignore = false;
        let mut timeout = None;
        for (attr, _span) in attributes {
            match attr {
                FuncAttribute::Init => unreachable!(),
                FuncAttribute::Test => {}
                FuncAttribute::ShouldPanic => should_panic = true,
                FuncAttribute::Ignore => ignore = true,
                FuncAttribute::Timeout(t) => timeout = Some(t.value),
            }
        }

        if check_fn_sig(&func.sig).is_err() || func.sig.inputs.len() > 1 {
            abort!(
                func.sig,
                "`#[test]` function must have signature `async fn(state: Type)` (async/parameter are optional)",
            );
        }

        if cfg!(not(feature = "embassy")) && func.sig.asyncness.is_some() {
            abort!(
                func.sig,
                "`#[test]` function can only be async if an async executor is enabled via feature",
            );
        }

        let input = if func.sig.inputs.len() == 1 {
            Some(extract_single_value_arg(&func.sig.inputs[0]))
            // NOTE we cannot check the argument type matches `init.state` at this point
        } else {
            None
        };

        TestFunc {
            cfgs: extract_cfgs(&func.attrs),
            asyncness: func.sig.asyncness.is_some(),
            func,
            input,
            should_panic,
            ignore,
            timeout,
        }
    }
}

pub(crate) enum Func {
    Init(InitFunc),
    Test(TestFunc),
    Other(FunctionWithAttributes),
}

impl From<FunctionWithAttributes> for Func {
    fn from(func: FunctionWithAttributes) -> Self {
        enum FuncKind {
            Init,
            Test,
        }
        let mut func_kind = None;
        for (attr, span) in &func.attributes {
            match attr {
                FuncAttribute::Init if func_kind.is_none() => func_kind = Some(FuncKind::Init),
                FuncAttribute::Test if func_kind.is_none() => func_kind = Some(FuncKind::Test),
                FuncAttribute::Init | FuncAttribute::Test => {
                    abort!(
                        span,
                        "A function can only be marked with one of `#[init]` or `#[test]`"
                    );
                }
                _ => {}
            }
        }

        match func_kind {
            Some(FuncKind::Init) => Func::Init(InitFunc::from(func)),
            Some(FuncKind::Test) => Func::Test(TestFunc::from(func)),
            None => Func::Other(func),
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

fn extract_single_value_arg(arg: &syn::FnArg) -> Type {
    if let syn::FnArg::Typed(pat) = arg {
        match &*pat.ty {
            syn::Type::Reference(_) => {}
            _ => return *pat.ty.clone(),
        }
    }
    abort!(arg, "parameter must be a single value, not a reference");
}
