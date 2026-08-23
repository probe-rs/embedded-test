// File copied and adapted from https://github.com/knurling-rs/defmt/blob/main/macros/src/attributes/panic_handler.rs
use proc_macro::TokenStream;
use proc_macro_error3::{abort, abort_call_site};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, ItemFn, ReturnType, Safety};

pub(crate) fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    if !args.is_empty() {
        abort_call_site!("`#[embedded_test::setup]` attribute takes no arguments");
    }

    let fun = parse_macro_input!(item as ItemFn);

    validate(&fun);

    codegen(&fun)
}

fn validate(fun: &ItemFn) {
    if fun.sig.constness.is_some()
        || fun.sig.asyncness.is_some()
        || matches!(fun.sig.safety, Safety::Unsafe(_))
        || fun.sig.abi.is_some()
        || !fun.sig.generics.params.is_empty()
        || fun.sig.generics.where_clause.is_some()
        || fun.sig.variadic.is_some()
        || !fun.sig.inputs.is_empty()
        || fun.sig.output != ReturnType::Default
    {
        abort!(fun.sig.ident, "function must have signature `fn() -> () `");
    }

    check_for_attribute_conflicts("setup", &fun.attrs, &["export_name", "no_mangle"]);
}

/// Checks if any attribute in `attrs_to_check` is in `reject_list` and returns a compiler error if there's a match
///
/// The compiler error will indicate that the attribute conflicts with `attr_name`
fn check_for_attribute_conflicts(
    attr_name: &str,
    attrs_to_check: &[Attribute],
    reject_list: &[&str],
) {
    for attr in attrs_to_check {
        if let Some(ident) = attr.path().get_ident() {
            let ident = ident.to_string();

            if reject_list.contains(&ident.as_str()) {
                abort!(
                    attr,
                    "`#[{}]` attribute cannot be used together with `#[{}]`",
                    attr_name,
                    ident
                )
            }
        }
    }
}

fn codegen(fun: &ItemFn) -> TokenStream {
    let attrs = &fun.attrs;
    let block = &fun.block;
    let ident = &fun.sig.ident;

    if cfg!(feature = "std") {
        // Export the setup function so that we can collect it using linkme when on std
        let ident_var = format_ident!("__{}_SETUP_SYM", ident.to_string().to_uppercase());
        quote!(
            #(#attrs)*
            #[inline(never)]
            fn #ident() {
                #block
            }

            #(#attrs)*
            #[embedded_test::export::hosting::distributed_slice(embedded_test::export::hosting::SETUP)]
            #[linkme(crate = embedded_test::export::hosting::linkme)]
            static #ident_var: fn() = #ident;
        )
    } else {
        quote!(
            #(#attrs)*
            #[export_name = "_embedded_test_setup"]
            #[inline(never)]
            fn #ident() {
                #block
            }
        )
    }
    .into()
}
