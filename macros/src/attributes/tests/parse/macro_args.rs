use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro::TokenStream;

#[derive(Debug, FromMeta)]
pub(crate) struct MacroArgs {
    pub executor: Option<syn::Expr>,
    pub default_timeout: Option<u32>,
}

impl MacroArgs {
    pub(crate) fn parse(args: TokenStream) -> Result<Self, syn::Error> {
        let attr_args = NestedMeta::parse_meta_list(args.into())?;
        let macro_args = MacroArgs::from_list(&attr_args)?;
        Ok(macro_args)
    }
}
