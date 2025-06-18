use crate::attributes::tests::parse::items::TestFunc;
use crate::attributes::tests::parse::Input;
use proc_macro2::Ident;
use quote::{format_ident, quote};

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

    // Generate a symbol name which is actually a JSON object describing the test so that probe-rs can parse it.
    let sym_name = format!(
        r#"{{"name":"{}","ignored":{},"should_panic":{}{}}}"#,
        json_escape(&test_name.to_string()),
        if ignore { "true" } else { "false" },
        if should_panic { "true" } else { "false" },
        if let Some(timeout) = timeout {
            format!(",\"timeout\":{}", timeout)
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

fn json_escape(string: &str) -> String {
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
