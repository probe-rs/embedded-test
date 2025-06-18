use items::{parse_item, Func, TestFunc};
use macro_args::{parse_args, MacroArgs};
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, Item, ItemMod, Type};

pub(crate) mod items;
pub(crate) mod macro_args;

pub(crate) struct Input {
    pub module_name: String,
    pub macro_args: MacroArgs,
    pub init: Option<items::InitFunc>,
    pub tests: Vec<items::TestFunc>,
    pub untouched_tokens: Vec<Item>,
}

impl Input {
    pub(crate) fn from_token_stream(
        args: TokenStream,
        input: TokenStream,
    ) -> Result<Self, syn::Error> {
        let macro_args = parse_args(args)?;

        let module: ItemMod = syn::parse(input)?;

        let items = if let Some(content) = module.content {
            content.1
        } else {
            return Err(parse::Error::new(
                module.span(),
                "module must be inline (e.g. `mod foo {}`)",
            ));
        };

        // Collect the init fn and all tests
        let mut init = None;
        let mut tests = vec![];
        let mut untouched_tokens = vec![];
        for item in items {
            match item {
                Item::Fn(f) => match parse_item(f)? {
                    Func::Init(init_func) => {
                        if init.is_some() {
                            return Err(parse::Error::new(
                                init_func.func.sig.ident.span(),
                                "only one `#[init]` function is allowed in a test module",
                            ));
                        }
                        init = Some(init_func);
                    }
                    Func::Test(test_func) => {
                        tests.push(test_func);
                    }
                },
                _ => {
                    untouched_tokens.push(item);
                }
            }
        }

        //Validate the argument type of the test function, now that the init function is parsed
        let expected_arg_type = init.as_ref().and_then(|i| i.state.as_ref());
        for test in &tests {
            validate_argument_type(test, expected_arg_type)?;
        }

        Ok(Self {
            module_name: module.ident.to_string(),
            macro_args,
            init,
            tests,
            untouched_tokens,
        })
    }
}

fn validate_argument_type(test: &TestFunc, expected_type: Option<&Type>) -> Result<(), syn::Error> {
    if let Some(actual_type) = test.input.as_ref() {
        if let Some(expected_type) = expected_type {
            if *actual_type != *expected_type {
                return Err(parse::Error::new(
                    actual_type.span(),
                    format!(
                        "this type must match `#[init]`s return type: {}",
                        type_ident(expected_type)
                    ),
                ));
            }
        } else {
            return Err(parse::Error::new(
                test.func.sig.span(),
                "no state was initialized/returned by `#[init]`; signature must be `fn()`",
            ));
        }
    }
    Ok(())
}
fn type_ident(ty: &syn::Type) -> String {
    let mut ident = String::new();
    let ty = format!("{}", quote!(#ty));
    ty.split_whitespace().for_each(|t| ident.push_str(t));
    ident
}
