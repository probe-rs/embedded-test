// Copied from https://github.com/knurling-rs/defmt/blob/main/firmware/defmt-test/macros/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse, spanned::Spanned, Attribute, Item, ItemFn, ItemMod, ReturnType, Type};

#[proc_macro_attribute]
pub fn tests(args: TokenStream, input: TokenStream) -> TokenStream {
    match tests_impl(args, input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn tests_impl(args: TokenStream, input: TokenStream) -> parse::Result<TokenStream> {
    if !args.is_empty() {
        return Err(parse::Error::new(
            Span::call_site(),
            "`#[test]` attribute takes no arguments",
        ));
    }

    let module: ItemMod = syn::parse(input)?;

    let items = if let Some(content) = module.content {
        content.1
    } else {
        return Err(parse::Error::new(
            module.span(),
            "module must be inline (e.g. `mod foo {}`)",
        ));
    };

    let mut init = None;
    let mut tests = vec![];
    let mut untouched_tokens = vec![];
    for item in items {
        match item {
            Item::Fn(mut f) => {
                let mut test_kind = None;
                let mut should_panic = false;
                let mut ignore = false;

                f.attrs.retain(|attr| {
                    if attr.path().is_ident("init") {
                        test_kind = Some(Attr::Init);
                        false
                    } else if attr.path().is_ident("test") {
                        test_kind = Some(Attr::Test);
                        false
                    } else if attr.path().is_ident("should_panic") {
                        should_panic = true;
                        false
                    } else if attr.path().is_ident("ignore") {
                        ignore = true;
                        false
                    } else {
                        true
                    }
                });

                let attr = match test_kind {
                    Some(it) => it,
                    None => {
                        return Err(parse::Error::new(
                            f.span(),
                            "function requires `#[init]` or `#[test]` attribute",
                        ));
                    }
                };

                match attr {
                    Attr::Init => {
                        if init.is_some() {
                            return Err(parse::Error::new(
                                f.sig.ident.span(),
                                "only a single `#[init]` function can be defined",
                            ));
                        }

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
                            ReturnType::Type(.., ty) => Some(ty.clone()),
                        };
                        let asyncness = f.sig.asyncness.is_some();
                        init = Some(Init {
                            func: f,
                            state,
                            asyncness,
                        });
                    }

                    Attr::Test => {
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
                            let arg = &f.sig.inputs[0];

                            // NOTE we cannot check the argument type matches `init.state` at this
                            // point
                            if let Some(ty) = get_arg_type(arg).cloned() {
                                Some(Input { ty })
                            } else {
                                // was not `&mut T`
                                return Err(parse::Error::new(
                                    arg.span(),
                                    "parameter must be of the type that init() returns",
                                ));
                            }
                        } else {
                            None
                        };

                        let asyncness = f.sig.asyncness.is_some();
                        tests.push(Test {
                            cfgs: extract_cfgs(&f.attrs),
                            func: f,
                            asyncness,
                            input,
                            should_panic,
                            ignore,
                        });
                    }
                }
            }

            _ => {
                untouched_tokens.push(item);
            }
        }
    }

    let krate = format_ident!("embedded_test");
    let ident = module.ident;
    let mut state_ty = None;
    let (init_fn, init_expr, init_is_async) = if let Some(ref init) = init {
        let init_func = &init.func;
        let init_ident = &init.func.sig.ident;
        state_ty = init.state.clone();

        (
            Some(quote!(#init_func)),
            Some(invoke(init_ident, vec![], init.asyncness)),
            init.asyncness,
        )
    } else {
        (None, None, false)
    };

    let mut unit_test_calls = vec![];
    let mut test_function_invokers = vec![];

    for test in &tests {
        let should_panic = test.should_panic;
        let ignore = test.ignore;
        let ident = &test.func.sig.ident;
        let ident_invoker = format_ident!("__{}_invoker", ident);
        let span = test.func.sig.ident.span();

        //TODO: if init func returns something, all functions must accept it as input?
        let mut args = vec![];

        if let Some(input) = test.input.as_ref() {
            if let Some(state) = &state_ty {
                if input.ty != **state {
                    return Err(parse::Error::new(
                        input.ty.span(),
                        format!(
                            "this type must match `#[init]`s return type: {}",
                            type_ident(state)
                        ),
                    ));
                }
                args.push(quote!(state));
            } else {
                return Err(parse::Error::new(
                    span,
                    "no state was initialized by `#[init]`; signature must be `fn()`",
                ));
            }
        }

        let run_call = invoke(ident, args, test.asyncness);

        let init_run_and_check = quote!(
            {
                 let state = #init_expr; // either init() or init().await
                 let outcome = #run_call; // either test(state), test(state).await, test(), or test().await
                 #krate::export::check_outcome(outcome);
            }
        );

        // The closure that will be called, if the test should be runned.
        // This closure has the signature () -> !, so it will never return.
        // The closure will signal the test result via semihosting exit/abort instead
        let entrypoint = if test.asyncness || init_is_async {
            // We need a utility function, so that embassy can create a task for us
            let cfgs = &test.cfgs;
            test_function_invokers.push(quote!(
                  #(#cfgs)*
                  #[embassy_executor::task]
                  async fn #ident_invoker() {
                      #init_run_and_check
                  }
            ));

            quote!(|| {
                let mut executor = ::embassy_executor::Executor::new();
                let executor = unsafe { __make_static(&mut executor) };
                executor.run(|spawner| {
                    spawner.must_spawn(#ident_invoker());
                })
            })
        } else {
            quote!(|| {
                #init_run_and_check
            })
        };

        unit_test_calls.push(quote! {
            const FULLY_QUALIFIED_FN_NAME: &str = concat!(module_path!(), "::", stringify!(#ident));
            test_funcs.push(#krate::export::Test{name: FULLY_QUALIFIED_FN_NAME, ignored: #ignore, should_panic: #should_panic, function: #entrypoint}).unwrap();
        });
    }

    let test_functions = tests.iter().map(|test| &test.func);
    let test_cfgs = tests.iter().map(|test| &test.cfgs);
    let maximal_number_tests = tests.len();

    Ok(quote!(
    #[cfg(test)]
    mod #ident {
        #(#untouched_tokens)*

        unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
            ::core::mem::transmute(t)
        }

        #[export_name = "main"]
        unsafe extern "C" fn __defmt_test_entry() -> ! {
            let mut test_funcs: #krate::export::Vec<#krate::export::Test, #maximal_number_tests> = #krate::export::Vec::new();
            #(
                #(#test_cfgs)*
                {
                    #unit_test_calls // pushes Test to test_funcs
                }
            )*

            #krate::export::run_tests(&mut test_funcs[..]);
        }

        #init_fn

        #(
            #test_functions
        )*

        #(
            #test_function_invokers
        )*
    })
        .into())
}

#[derive(Clone, Copy)]
enum Attr {
    Init,
    Test,
}

struct Init {
    func: ItemFn,
    state: Option<Box<Type>>,
    asyncness: bool,
}

struct Test {
    func: ItemFn,
    cfgs: Vec<Attribute>,
    input: Option<Input>,
    should_panic: bool,
    ignore: bool,
    asyncness: bool,
}

struct Input {
    ty: Type,
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

fn get_arg_type(arg: &syn::FnArg) -> Option<&Type> {
    if let syn::FnArg::Typed(pat) = arg {
        match &*pat.ty {
            syn::Type::Reference(_) => None,
            _ => Some(&pat.ty),
        }
    } else {
        None
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

fn type_ident(ty: impl AsRef<syn::Type>) -> String {
    let mut ident = String::new();
    let ty = ty.as_ref();
    let ty = format!("{}", quote!(#ty));
    ty.split_whitespace().for_each(|t| ident.push_str(t));
    ident
}

fn invoke(
    func: &proc_macro2::Ident,
    args: Vec<proc_macro2::TokenStream>,
    asyncness: bool,
) -> proc_macro2::TokenStream {
    if asyncness {
        quote!(#func(#(#args),*).await)
    } else {
        quote!(#func(#(#args),*))
    }
}
