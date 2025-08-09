use crate::attributes::tests::validate::{InitFunc, TestFunc};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

fn invoke(func: &ItemFn, args: Vec<TokenStream>) -> TokenStream {
    let ident = &func.sig.ident;
    if func.sig.asyncness.is_some() {
        quote!(#ident(#(#args),*).await)
    } else {
        quote!(#ident(#(#args),*))
    }
}

/// Generate a code block ( in { ... }) to call the init function (if provided), call the test function and check the outcome.
pub(crate) fn call_test_fn(test_func: &TestFunc, init_func: Option<&InitFunc>) -> TokenStream {
    let init_expr = if let Some(init) = init_func {
        invoke(&init.func, vec![])
    } else {
        quote!(())
    };

    let run_call = if test_func.input.is_some() {
        invoke(&test_func.func, vec![quote!(state)])
    } else {
        invoke(&test_func.func, vec![])
    };

    quote!(
        {
            let outcome;
            {
                let state = #init_expr; // either init() or init().await or ()
                outcome = #run_call; // either test(state), test(state).await, test(), or test().await
            }
            embedded_test::export::check_outcome(outcome);
        }
    )
}
