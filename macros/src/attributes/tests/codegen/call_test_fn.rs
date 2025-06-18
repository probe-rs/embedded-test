use crate::attributes::tests::codegen::Invoke;
use crate::attributes::tests::parse::items::{InitFunc, TestFunc};
use proc_macro2::TokenStream;
use quote::quote;

/// Generate a code block ( in { ... }) to call the init function (if provided), call the test function and check the outcome.
pub(crate) fn call_test_fn(test_func: &TestFunc, init_func: Option<&InitFunc>) -> TokenStream {
    let init_expr = if let Some(init) = init_func {
        init.invoke_with(vec![])
    } else {
        quote!(())
    };

    let run_call = if test_func.input.is_some() {
        test_func.invoke_with(vec![quote!(state)])
    } else {
        test_func.invoke_with(vec![])
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
