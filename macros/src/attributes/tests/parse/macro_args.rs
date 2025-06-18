use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro::TokenStream;

#[derive(Debug, FromMeta)]
pub(crate) struct MacroArgs {
    pub executor: Option<syn::Expr>,
    pub default_timeout: Option<u32>,
    pub setup: Option<syn::Expr>,
}

pub(crate) fn parse_args(args: TokenStream) -> Result<MacroArgs, syn::Error> {
    let attr_args = NestedMeta::parse_meta_list(args.into())?;
    let macro_args = MacroArgs::from_list(&attr_args)?;

    #[cfg(not(all(feature = "embassy", feature = "external-executor")))]
    if macro_args.executor.is_some() {
        return Err(syn::parse::Error::new(
            proc_macro2::Span::call_site(),
            "`#[embedded_test::tests]` attribute doesn't take an executor unless the features `embassy` and `external-executor` are enabled",
        ));
    }

    Ok(macro_args)
}
