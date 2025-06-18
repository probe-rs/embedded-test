use crate::attributes::tests::codegen::call_test_fn::call_test_fn;
use crate::attributes::tests::codegen::export_sym::export_sym;
use crate::attributes::tests::codegen::wrap_with_executor::wrap_with_executor;
use crate::attributes::tests::parse::items::{InitFunc, TestFunc};
use crate::attributes::tests::parse::Input;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) mod call_test_fn;
pub(crate) mod export_sym;
pub(crate) mod wrap_with_executor;

pub(crate) trait Invoke {
    fn invoke_with(&self, args: Vec<TokenStream>) -> TokenStream;
}

impl Invoke for TestFunc {
    fn invoke_with(&self, args: Vec<TokenStream>) -> TokenStream {
        let ident = &self.func.sig.ident;
        if self.asyncness {
            quote!(#ident(#(#args),*).await)
        } else {
            quote!(#ident(#(#args),*))
        }
    }
}

impl Invoke for InitFunc {
    fn invoke_with(&self, args: Vec<TokenStream>) -> TokenStream {
        let ident = &self.func.sig.ident;
        if self.asyncness {
            quote!(#ident(#(#args),*).await)
        } else {
            quote!(#ident(#(#args),*))
        }
    }
}

pub(crate) fn test(test: &TestFunc, input: &Input) -> TokenStream {
    let ident = &test.func.sig.ident;
    let ident_entrypoint = format_ident!("__{}_entrypoint", ident);
    let cfgs = &test.cfgs;
    let test_func = &test.func;
    let mut embassy_task = None;

    // Generate the code block that will call init, run the test and check the outcome.
    let mut test_invocation = call_test_fn(test, input.init.as_ref());

    let init_is_async = input.init.as_ref().map(|i| i.asyncness).unwrap_or_default();

    // If the test is async or the init function is async, we need to wrap the test invocation in an executor.
    // Result is still a block
    if test.asyncness || init_is_async {
        let additional_output;
        (test_invocation, additional_output) =
            wrap_with_executor(test, input.macro_args.executor.as_ref(), test_invocation);
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
    let sym = export_sym(input, test, ident_entrypoint);

    quote! {

        #test_func

        #embassy_task

        #test_entrypoint

        #sym

    }
}
