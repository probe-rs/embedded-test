use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemMod};

mod codegen;
mod parse;
mod validate;

pub(crate) fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let macro_args = match parse::MacroArgs::parse(args) {
        Ok(args) => args,
        Err(e) => return syn::Error::new(e.span(), e).into_compile_error().into(),
    };

    let module = match parse::Module::try_from(parse_macro_input!(input as ItemMod)) {
        Ok(module) => module,
        Err(err) => return err.into_compile_error().into(),
    };
    let validated_module = match validate::ValidatedModule::from_module_and_args(module, macro_args)
    {
        Ok(valid_mod) => valid_mod,
        Err(err) => return err.into_compile_error().into(),
    };

    let untouched_tokens = &validated_module.untouched_tokens;

    let preamble = {
        if cfg!(feature = "ariel-os") {
            // Without this, nothing would link the Ariel OS startup code unless
            // either `init()` or any of the tests is async. It then fails at link time.
            quote!(
                use ariel_os as _;
            )
        } else {
            quote!()
        }
    };

    let tests = validated_module
        .tests
        .iter()
        .map(|test| codegen::test(test, &validated_module));
    let init_fns = validated_module.init_funcs.values().map(|i| &i.func);

    let mod_name = format_ident!("{}", validated_module.module_name);
    quote!(
        #preamble

        mod #mod_name {
            #(#untouched_tokens)*

            #(#init_fns)*

            #(#tests)*
        }
    )
    .into()
}
