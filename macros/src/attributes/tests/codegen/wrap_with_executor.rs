use crate::attributes::tests::validate::TestFunc;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Expr;

/// wraps the provided block in an embassy task, optionally using a custom executor.
/// Returns a block that spawns the task and also returns the task function itself.
pub(crate) fn wrap_with_executor(
    test: &TestFunc,
    custom_executor: Option<&Expr>,
    block: TokenStream,
) -> (TokenStream, TokenStream) {
    let task_attribute = if cfg!(feature = "ariel-os") {
        quote!(ariel_os::task)
    } else {
        quote!(embedded_test::export::task)
    };

    let executor_constructor = if let Some(executor) = custom_executor {
        quote! {
            #executor
        }
    } else {
        quote! {
            embedded_test::export::Executor::new()
        }
    };

    let cfgs = &test.cfgs;
    let ident_invoker = format_ident!("__{}_invoker", test.func.sig.ident);

    // We need to create a new function annotated with the task attribute, to spawn an async task
    let task_fn = quote!(
          #(#cfgs)*
          #[#task_attribute]
          #[doc(hidden)]
          async fn #ident_invoker() {
              #block
          }
    );

    let spawn_invoker = if cfg!(feature = "embassy-09") {
        quote!( spawn(#ident_invoker()).unwrap() )
    } else if cfg!(feature = "embassy-010") {
        quote!( spawn(#ident_invoker().unwrap()) )
    } else {
        panic!("must select embassy-09 or embassy-10 feature");
    };

    let spawner_block = if cfg!(feature = "ariel-os") {
        quote!( {
            ariel_os::asynch::spawner().#spawn_invoker;
            ariel_os::thread::park();
            unreachable!();
        })
    } else {
        quote!( {
            let mut executor = #executor_constructor;
            unsafe fn __make_static<T>(t: &mut T) -> &'static mut T {
                ::core::mem::transmute(t)
            }
            let executor = unsafe { __make_static(&mut executor) };
            executor.run(|spawner| {
                spawner.#spawn_invoker;
            })
        })
    };

    (spawner_block, task_fn)
}
