use proc_macro::TokenStream;
use proc_macro_error2::abort;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemMod};

mod codegen;
mod parse;
mod validate;

pub(crate) fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let macro_args = match parse::MacroArgs::parse(args) {
        Ok(args) => args,
        Err(e) => abort!(e),
    };

    let module = parse::Module::from(parse_macro_input!(input as ItemMod));
    let validated_module = validate::ValidatedModule::from_module_and_args(module, macro_args);

    let untouched_tokens = &validated_module.untouched_tokens;
    let tests = validated_module
        .tests
        .iter()
        .map(|test| codegen::test(test, &validated_module));
    let init_fns = validated_module.init_funcs.values().map(|i| &i.func);

    let mod_name = format_ident!("{}", validated_module.module_name);
    quote!(
        mod #mod_name {
            #(#untouched_tokens)*

            #(#init_fns)*

            #(#tests)*
        }
    )
    .into()
}
