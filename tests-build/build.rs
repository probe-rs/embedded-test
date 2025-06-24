use glob::{GlobResult, glob};
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=cases");

    let pass_tests = glob("cases/pass/*.rs")
        .expect("Failed to read glob pattern")
        .map(|p| generate_test_function(p, true))
        .collect::<String>();
    let fail_tests = glob("cases/fail/*.rs")
        .expect("Failed to read glob pattern")
        .map(|p| generate_test_function(p, false))
        .collect::<String>();

    let test_cases_module = format!(
        "mod pass {{
{pass_tests}
}}

mod fail {{
{fail_tests}
}}"
    );

    std::fs::write(
        PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("test_cases.rs"),
        test_cases_module,
    )
    .unwrap();
}

fn generate_test_function(path: GlobResult, should_pass: bool) -> String {
    let path = path.expect("path contents could not be read");
    let test_name = path
        .file_stem()
        .expect("path file stem failed")
        .display()
        .to_string();
    let path = std::path::absolute(path)
        .expect("path absolute failed")
        .display()
        .to_string();

    format!(
        "#[test]
fn {test_name}() {{
    crate::run_test(&std::path::Path::new(r\"{path}\"), {should_pass});
}}"
    )
}
