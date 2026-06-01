use crate::attributes::tests::codegen::call_test_fn::call_test_fn;
use crate::attributes::tests::codegen::export_sym::export_sym;
use crate::attributes::tests::codegen::wrap_with_executor::wrap_with_executor;
use crate::attributes::tests::validate::{TestFunc, ValidatedModule};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn test(test: &TestFunc, module: &ValidatedModule) -> TokenStream {
    let ident = &test.func.sig.ident;
    let ident_body = format_ident!("__{}_body", ident);
    let ident_entrypoint = format_ident!("__{}_entrypoint", ident);
    let cfgs = &test.cfgs;

    // Keep the real test body in a hidden helper so the original name
    // is free for an IDE-visible `#[test]` wrapper.
    // Only the ident changes; Attributes and signature are preserved.
    let mut test_func = test.func.clone();
    test_func.sig.ident = ident_body.clone();

    let mut embassy_task = None;

    // Generate the code block that will call init, run the test and check the outcome.
    let init = module.init_function_for_test(test);
    let mut test_invocation = call_test_fn(test, &test_func, init);

    let init_is_async = init.map(|i| i.asyncness).unwrap_or_default();

    // If the test is async or the init function is async, we need to wrap the test invocation in an executor.
    // Result is still a block
    if test.asyncness || init_is_async {
        let additional_output;
        (test_invocation, additional_output) =
            wrap_with_executor(test, module.macro_args.executor.as_ref(), test_invocation);
        embassy_task = Some(additional_output);
    }

    // Now generate an entrypoint function that will be called by the test runner.
    // This function has the signature () -> !, so it will never return.
    // Instead, it will signal the test result via semihosting exit/abort instead
    let test_entrypoint = quote!(
        #[doc(hidden)]
        #(#cfgs)*
        fn #ident_entrypoint() -> ! {
           #test_invocation
        }
    );

    // A static symbol that will be exported that describes the test and can be parsed by probe-rs.
    let sym = export_sym(test, ident_entrypoint, module.macro_args.default_timeout);

    // Generate a wrapper that rust-analyzer can recognize as a runnable test.
    // The real embedded-test body is kept in a hidden helper so the editor-facing
    // test item stays a normal zero-argument #[test] function.
    let test_wrapper = quote!(
        #[doc(hidden)]
        #[allow(dead_code)]
        #(#cfgs)*
        #[::core::prelude::v1::test]
        fn #ident() {}
    );

    quote! {

        #test_func

        #test_wrapper

        #embassy_task

        #test_entrypoint

        #sym

    }
}
