use crate::attributes::tests::parse::FunctionWithAttributes;
use proc_macro_error2::abort;
use syn::{Item, ItemMod};

pub(crate) struct Module {
    pub name: String,
    pub functions: Vec<FunctionWithAttributes>,
    pub untouched_tokens: Vec<Item>,
}

impl From<ItemMod> for Module {
    fn from(module: ItemMod) -> Self {
        let Some((_, items)) = module.content else {
            abort!(module, "module must be inline (e.g. `mod foo {}`)",);
        };

        let mut untouched_tokens = vec![];
        let mut functions = vec![];
        for item in items {
            match item {
                Item::Fn(f) => functions.push(FunctionWithAttributes::from(f)),
                _ => untouched_tokens.push(item),
            }
        }

        Self {
            name: module.ident.to_string(),
            functions,
            untouched_tokens,
        }
    }
}
