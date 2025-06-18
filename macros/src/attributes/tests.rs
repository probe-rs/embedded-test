use proc_macro::TokenStream;
use quote::{format_ident, quote};

mod codegen;
mod parse;

pub(crate) fn expand(args: TokenStream, input: TokenStream) -> syn::parse::Result<TokenStream> {
    let input = parse::Input::from_token_stream(args, input)?;

    let untouched_tokens = &input.untouched_tokens;
    let init_fn = input.init.as_ref().map(|i| &i.func);
    let tests = input.tests.iter().map(|test| codegen::test(test, &input));

    let mod_name = format_ident!("{}", input.module_name);
    Ok(quote!(
        #[cfg(test)]
        mod #mod_name {
            #(#untouched_tokens)*

            #init_fn

            #(#tests)*
        }
    )
    .into())
}
