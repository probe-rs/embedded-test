mod manifest;

use crate::manifest::Manifest;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

include!(concat!(env!("OUT_DIR"), "/test_cases.rs"));

#[allow(dead_code)]
fn run_test(test_file: &Path, should_pass: bool) {
    let test_name = test_file
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Failed to get file stem")
        .to_string();
    let temp_dir = Path::new(env!("OUT_DIR")).join(test_name);
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory for test");
    }

    let mut manifest = Manifest::from_rust_file(test_file).unwrap();
    manifest.fix(test_file).unwrap();
    let manifest_path = temp_dir.join("Cargo.toml");
    manifest.write(&manifest_path).unwrap();

    let output = Command::new("cargo")
        .args(&[
            "test",
            "--manifest-path",
            manifest_path.to_str().unwrap(),
            //"--quiet",
            "--color=never",
            //"--message-format=json",
            "--no-run",
            "--target",
            "riscv32imac-unknown-none-elf", //TOOD: other targets?
        ])
        .output()
        .unwrap();

    if should_pass {
        if output.status.success() {
            return;
        } else {
            std::io::stdout().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
            panic!(
                "Test {} failed, but it was expected to pass.",
                test_file.display()
            );
        }
    } else {
        if output.status.success() {
            std::io::stdout().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
            panic!(
                "Test {} passed, but it was expected to fail.",
                test_file.display()
            );
        } else {
            // TODO: normalize stderr output, so that we can compare the full output not only do a haystack search
            let expected_stderr = test_file.with_extension("stderr");
            let expected_stderr = if fs::exists(&expected_stderr).unwrap() {
                Some(fs::read_to_string(&expected_stderr).unwrap())
            } else {
                None
            };
            let actual_stderr = String::from_utf8_lossy(&output.stderr);
            match (expected_stderr, actual_stderr) {
                (Some(expected), actual) if actual.contains(&expected) => {
                    return; // The test failed as expected
                }
                (Some(expected), actual) => {
                    panic!(
                        "Test {} failed, but the expected sentence was not found in the stderr output.\nExpected Sentence:\n{}\nActual:\n{}",
                        test_file.display(),
                        expected,
                        actual
                    );
                }
                (None, actual) => {
                    panic!(
                        "Test {} failed, but no expected stderr output was provided.\nActual:\n{}",
                        test_file.display(),
                        actual
                    );
                }
            }
        }
    }
}
