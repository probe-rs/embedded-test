use crate::attributes::tests::parse::Module;
use crate::attributes::tests::parse::{FunctionWithAttributes, MacroArgs};
use crate::attributes::tests::validate::{AnnotatedFunction, InitFunc, OtherFunc, TestFunc};
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::Item;

pub(crate) struct ValidatedModule {
    pub module_name: String,
    pub default_init: Option<String>,
    pub init_funcs: HashMap<String, InitFunc>,
    pub tests: Vec<TestFunc>,
    pub untouched_tokens: Vec<Item>,
    pub macro_args: MacroArgs,
}

impl ValidatedModule {
    pub(crate) fn from_module_and_args(module: Module, macro_args: MacroArgs) -> syn::Result<Self> {
        let Module {
            name: module_name,
            functions,
            mut untouched_tokens,
        } = module;

        let (default_init_func, tests, other_funcs) = categorize_functions(functions)?;
        let default_init = default_init_func.as_ref().map(|i| i.name.clone());

        let (init_funcs, other_funcs) =
            collect_init_functions(&tests, default_init_func, other_funcs)?;

        untouched_tokens.extend(other_funcs.into_iter().map(|f| Item::Fn(f.0.func))); // Add unused functions back to untouched tokens

        let m = ValidatedModule {
            module_name,
            init_funcs,
            tests,
            untouched_tokens,
            macro_args,
            default_init,
        };
        m.validate_functions()?;
        m.validate_macro_args()?;
        Ok(m)
    }

    pub(crate) fn init_function_for_test(&self, test: &TestFunc) -> Option<&InitFunc> {
        if let Some(custom_init) = test.custom_init.as_ref() {
            return Some(self.init_funcs.get(&custom_init.to_string()).unwrap());
        }

        self.default_init
            .as_ref()
            .map(|i| self.init_funcs.get(i.as_str()).unwrap())
    }

    fn validate_functions(&self) -> syn::Result<()> {
        // Validate the argument type of the test function, now that the init function is parsed
        for test in &self.tests {
            validate_argument_type(test, self.init_function_for_test(test))?;
        }
        Ok(())
    }

    fn validate_macro_args(&self) -> syn::Result<()> {
        // Validate a custom executor is only provided if the feature is enabled
        if cfg!(not(all(feature = "embassy", feature = "external-executor")))
            && self.macro_args.executor.is_some()
        {
            return Err(syn::Error::new(Span::call_site(),
                "`#[embedded_test::tests]` attribute doesn't take an executor unless the features `embassy` and `external-executor` are enabled",
            ));
        }

        // Validate a custom executor is provided if needed and at least one test/init is async
        if cfg!(feature = "external-executor")
            && self.macro_args.executor.is_none()
            && (self.tests.iter().any(|test| test.asyncness)
                || self.init_funcs.iter().any(|(_, init)| init.asyncness))
        {
            return Err(syn::Error::new(Span::call_site(),
                "async test/init func requires that an executor is provided via `#[embedded_test::tests(executor = ...)]` because the feature `external-executor` is enabled",
            ));
        }
        Ok(())
    }
}

fn categorize_functions(
    functions: Vec<FunctionWithAttributes>,
) -> syn::Result<(Option<InitFunc>, Vec<TestFunc>, Vec<OtherFunc>)> {
    let mut init_func = None;
    let mut tests = vec![];
    let mut other_funcs = vec![];

    for func in functions {
        match AnnotatedFunction::try_from(func)? {
            AnnotatedFunction::Init(i) if init_func.is_none() => init_func = Some(i),
            AnnotatedFunction::Init(i) => {
                return Err(syn::Error::new(
                    i.func.sig.span(),
                    "only one `#[init]` function is allowed in a test module",
                ));
            }
            AnnotatedFunction::Test(t) => tests.push(t),
            AnnotatedFunction::Other(f) => other_funcs.push(f),
        }
    }

    Ok((init_func, tests, other_funcs))
}

fn collect_init_functions(
    tests: &[TestFunc],
    default_init: Option<InitFunc>,
    mut other_funcs: Vec<OtherFunc>,
) -> syn::Result<(HashMap<String, InitFunc>, Vec<OtherFunc>)> {
    let mut map = HashMap::new();

    if let Some(init_func) = default_init {
        map.insert(init_func.name.clone(), init_func);
    }

    for test in tests {
        if let Some(init_fn_ident) = test.custom_init.as_ref() {
            let init_fn_name = init_fn_ident.to_string();

            if map.contains_key(&init_fn_name) {
                continue;
            }

            if let Some(pos) = other_funcs
                .iter()
                .position(|f| f.0.func.sig.ident == init_fn_name)
            {
                let init_func = InitFunc::try_from(other_funcs.remove(pos).0)?;
                map.insert(init_fn_name, init_func);
            } else {
                return Err(syn::Error::new(
                    init_fn_ident.span(),
                    format!(
                        "custom init function `{}` not found in the module",
                        init_fn_name
                    ),
                ));
            }
        }
    }
    Ok((map, other_funcs))
}

fn validate_argument_type(test: &TestFunc, init_func: Option<&InitFunc>) -> syn::Result<()> {
    let init_func = init_func.map(|i| (i.name.as_str(), i.state.as_ref()));

    match (&test.input, init_func) {
        (Some(_), None) => {
            return Err(syn::Error::new(
                test.func.sig.span(),
                "this test function has an argument but no `#[init]` function was provided",
            ));
        }
        (Some(_), Some((init_fn_name, None))) => {
            return Err(syn::Error::new(
                test.func.sig.span(),
                format!(
                    "this test function has an argument but the init function `{}` does not return a state",
                    init_fn_name
            )));
        }
        (Some(actual_type), Some((init_fn_name, Some(expected_type))))
            if actual_type != expected_type =>
        {
            return Err(syn::Error::new(
                actual_type.span(),
                format!(
                    "this type must match the return type `{}` of the init function `{}`",
                    type_ident(expected_type),
                    init_fn_name
                ),
            ));
        }
        _ => {}
    }
    Ok(())
}

fn type_ident(ty: &syn::Type) -> String {
    let mut ident = String::new();
    let ty = format!("{}", quote!(#ty));
    ty.split_whitespace().for_each(|t| ident.push_str(t));
    ident
}
