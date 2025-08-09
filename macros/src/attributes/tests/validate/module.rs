use crate::attributes::tests::parse::Module;
use crate::attributes::tests::parse::{FunctionWithAttributes, MacroArgs};
use crate::attributes::tests::validate::{Func, InitFunc, TestFunc};
use proc_macro_error2::{abort, abort_call_site};
use quote::quote;
use syn::{Item, Type};

pub(crate) struct ValidatedModule {
    pub module_name: String,
    pub init_func: Option<InitFunc>,
    pub tests: Vec<TestFunc>,
    pub untouched_tokens: Vec<Item>,
    pub macro_args: MacroArgs,
}

impl ValidatedModule {
    pub(crate) fn from_module_and_args(module: Module, macro_args: MacroArgs) -> Self {
        let Module {
            name: module_name,
            functions,
            untouched_tokens,
        } = module;

        let (init_func, tests) = categorize_functions(functions);

        let m = ValidatedModule {
            module_name,
            init_func,
            tests,
            untouched_tokens,
            macro_args,
        };
        m.validate_functions();
        m.validate_macro_args();
        m
    }

    fn validate_functions(&self) {
        // Validate the argument type of the test function, now that the init function is parsed
        let expected_arg_type = self.init_func.as_ref().and_then(|i| i.state.as_ref());
        for test in &self.tests {
            validate_argument_type(test, expected_arg_type);
        }
    }

    fn validate_macro_args(&self) {
        // Validate a custom executor is only provided if the feature is enabled
        if cfg!(not(all(feature = "embassy", feature = "external-executor")))
            && self.macro_args.executor.is_some()
        {
            abort_call_site!(
                "`#[embedded_test::tests]` attribute doesn't take an executor unless the features `embassy` and `external-executor` are enabled",
            );
        }

        // Validate a custom executor is provided if needed and at least one test/init is async
        if cfg!(feature = "external-executor")
            && self.macro_args.executor.is_none()
            && (self.tests.iter().any(|test| test.asyncness)
                || self.init_func.as_ref().is_some_and(|i| i.asyncness))
        {
            abort_call_site!(
                "async test/init func requires that an executor is provided via `#[embedded_test::tests(executor = ...)] because the feature `external-executor` is enabled",
            );
        }
    }
}

fn categorize_functions(
    functions: Vec<FunctionWithAttributes>,
) -> (Option<InitFunc>, Vec<TestFunc>) {
    let mut init_func = None;
    let mut tests = vec![];

    for func in functions {
        match Func::from(func) {
            Func::Init(i) if init_func.is_none() => init_func = Some(i),
            Func::Init(i) => {
                abort!(
                    i.func.sig,
                    "only one `#[init]` function is allowed in a test module",
                );
            }
            Func::Test(t) => tests.push(t),
            Func::Other(f) => {
                abort!(
                    f.func.sig,
                    "functions in a test module must be marked with `#[init]` or `#[test]`",
                );
                //Note: once this is no longer the case, validate the args instead!
            }
        }
    }

    (init_func, tests)
}

fn validate_argument_type(test: &TestFunc, expected_type: Option<&Type>) {
    if let Some(actual_type) = test.input.as_ref() {
        if let Some(expected_type) = expected_type {
            if *actual_type != *expected_type {
                abort!(
                    actual_type,
                    "this type must match `#[init]`s return type: {}",
                    type_ident(expected_type)
                );
            }
        } else {
            abort!(
                test.func.sig,
                "no state was initialized/returned by `#[init]`; signature must be `fn()`",
            );
        }
    }
}

fn type_ident(ty: &syn::Type) -> String {
    let mut ident = String::new();
    let ty = format!("{}", quote!(#ty));
    ty.split_whitespace().for_each(|t| ident.push_str(t));
    ident
}
