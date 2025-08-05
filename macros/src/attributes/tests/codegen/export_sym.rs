use crate::attributes::tests::parse::items::TestFunc;
use crate::attributes::tests::parse::Input;
use proc_macro::Span;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::hash::{DefaultHasher, Hash as _, Hasher as _};

pub(crate) fn export_sym(
    input: &Input,
    test: &TestFunc,
    ident_entrypoint: Ident,
) -> proc_macro2::TokenStream {
    let cfgs = &test.cfgs;
    let should_panic = test.should_panic;
    let ignore = test.ignore;
    let test_name = &test.func.sig.ident;
    let ident_var = format_ident!("__{}_SYM", test_name.to_string().to_uppercase());
    let timeout = test.timeout.or(input.macro_args.default_timeout);

    if cfg!(feature = "std") {
        // Export test as struct so that we can collect it using linkme when on std
        let timeout = if let Some(timeout) = timeout {
            quote!(Some(#timeout))
        } else {
            quote!(None)
        };
        quote!(
            #(#cfgs)*
            #[embedded_test::export::hosting::distributed_slice(embedded_test::export::hosting::TESTS)]
            #[linkme(crate= embedded_test::export::hosting::linkme)]
                static #ident_var: embedded_test::export::hosting::Test = embedded_test::export::hosting::Test {
                    name:  concat!(module_path!(), "::", stringify!(#test_name)),
                    function: #ident_entrypoint,
                    should_panic: #should_panic,
                    ignored: #ignore,
                    timeout: #timeout,
            };
        )
    } else {
        // Generate a symbol name which is actually a JSON object describing the test so that probe-rs can parse it.

        let sym_name = format!(
            r#"{{"disambiguator":{},"name":"{}","ignored":{},"should_panic":{}{}}}"#,
            _crate_local_disambiguator(), // disambiguator is needed to allow multiple identical test in different modules
            _json_escape(&test_name.to_string()),
            if ignore { "true" } else { "false" },
            if should_panic { "true" } else { "false" },
            if let Some(timeout) = timeout {
                format!(",\"timeout\":{timeout}")
            } else {
                String::new()
            }
        );

        // Unfortunately the module path can not be extracted from the Span yet.
        // At least on stable rust. Tracking issue: https://github.com/rust-lang/rust/issues/54725
        // As a workaround we use `module_path!()` to get the module path at runtime.
        quote!(
            #(#cfgs)*
            //#[used]
            //#[no_mangle]
            #[link_section = ".embedded_test.tests"]
            #[export_name = #sym_name]
            static #ident_var: (fn()->!,&'static str) = (#ident_entrypoint, module_path!());
        )
    }
}

fn _hash(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}
pub(crate) fn _crate_local_disambiguator() -> u64 {
    //copied from defmt ;)
    // We want a deterministic, but unique-per-macro-invocation identifier. For that we
    // hash the call site `Span`'s debug representation, which contains a counter that
    // should disambiguate macro invocations within a crate.
    _hash(&format!("{:?}", Span::call_site()))
}

fn _json_escape(string: &str) -> String {
    use std::fmt::Write;
    let mut escaped = String::new();
    for c in string.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '\"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            c if c.is_control() || c == '@' => write!(escaped, "\\u{:04x}", c as u32).unwrap(),
            c => escaped.push(c),
        }
    }
    escaped
}
