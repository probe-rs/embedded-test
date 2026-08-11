use crate::attributes::tests::parse::FunctionWithAttributes;
use syn::{spanned::Spanned, Item, ItemMod};

pub(crate) struct Module {
    pub name: String,
    pub functions: Vec<FunctionWithAttributes>,
    pub untouched_tokens: Vec<Item>,
}

impl TryFrom<ItemMod> for Module {
    type Error = syn::Error;

    fn try_from(module: ItemMod) -> syn::Result<Self> {
        let Some((_, items)) = module.content else {
            return Err(syn::Error::new(
                module.span(),
                "module must be inline (e.g. `mod foo {}`)",
            ));
        };

        let mut untouched_tokens = vec![];
        let mut functions = vec![];
        for item in items {
            match item {
                Item::Fn(f) => functions.push(FunctionWithAttributes::try_from(f)?),
                _ => untouched_tokens.push(item),
            }
        }

        Ok(Self {
            name: module.ident.to_string(),
            functions,
            untouched_tokens,
        })
    }
}
