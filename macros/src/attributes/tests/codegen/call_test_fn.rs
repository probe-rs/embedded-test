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
pub(crate) fn call_test_fn(
    test_func: &TestFunc,
    func_ident: &syn::Ident,
    init_func: Option<&InitFunc>,
) -> TokenStream {
    let init_expr = if let Some(init) = init_func {
        invoke(&init.func, vec![])
    } else {
        quote!(())
    };

    let run_call = match (test_func.input.is_some(), test_func.asyncness) {
        (true, true) => quote!(#func_ident(state).await),
        (true, false) => quote!(#func_ident(state)),
        (false, true) => quote!(#func_ident().await),
        (false, false) => quote!(#func_ident()),
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
