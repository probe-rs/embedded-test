mod build_rs;
mod manifest;
mod workspace_manifest;

use crate::manifest::Manifest;
use crate::workspace_manifest::WorkspaceManifest;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::sync::Once;

include!(concat!(env!("OUT_DIR"), "/test_cases.rs"));

const TEMP_TARGET_DIR_NAME: &str = "embedded-test-tests-target";
const ESP_TARGET_DIR_NAME: &str = "embedded-test-tests-target-esp";
const ESP_TARGET_TRIPLE: &str = "riscv32imac-unknown-none-elf";
const ESP_TARGET_PROFILE: &str = "debug";

fn lockfile_source_for_manifest(manifest: &Manifest) -> Option<&'static str> {
    if manifest.dependencies.contains_key("esp-hal")
        || manifest.dependencies.contains_key("esp-rtos")
        || manifest.dependencies.contains_key("embassy-executor")
    {
        Some(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../examples/esp32c6/Cargo.lock"
        ))
    } else {
        None
    }
}

fn write_lockfile_if_needed(manifest: &Manifest, manifest_path: &Path) -> anyhow::Result<()> {
    if let Some(source_lockfile) = lockfile_source_for_manifest(manifest) {
        let target_lockfile = manifest_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Manifest path has no parent directory"))?
            .join("Cargo.lock");
        fs::copy(source_lockfile, target_lockfile)?;
    }

    Ok(())
}

fn combined_output(output: &std::process::Output) -> String {
    format!(
        "{}{}{}",
        String::from_utf8_lossy(&output.stdout),
        if output.stdout.is_empty() { "" } else { "\n" },
        String::from_utf8_lossy(&output.stderr)
    )
}

fn normalize_line_endings(output: &str) -> String {
    output.replace("\r\n", "\n").replace('\r', "\n")
}

fn copy_dir_recursively(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursively(&path, &dest_path)?
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn esp_rom_sys_shared_build_dir() -> std::path::PathBuf {
    std::env::temp_dir()
        .join(ESP_TARGET_DIR_NAME)
        .join(ESP_TARGET_TRIPLE)
        .join(ESP_TARGET_PROFILE)
        .join("build")
}

fn test_target_dir(test_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join(TEMP_TARGET_DIR_NAME)
        .join(test_name)
}

fn copy_prebuilt_esp_rom_sys_artifacts(cargo_target_dir: &Path) {
    let shared_build_dir = esp_rom_sys_shared_build_dir();
    if !shared_build_dir.exists() {
        return;
    }

    let per_test_build_dir = cargo_target_dir
        .join(ESP_TARGET_TRIPLE)
        .join(ESP_TARGET_PROFILE)
        .join("build");
    if per_test_build_dir.exists() {
        return;
    }

    let _ = copy_dir_recursively(&shared_build_dir, &per_test_build_dir);
}

static ESP_ROM_SYS_BUILD: Once = Once::new();

fn pre_build_esp_rom_sys() {
    ESP_ROM_SYS_BUILD.call_once(|| {
        let shared_target_dir = std::env::temp_dir().join(ESP_TARGET_DIR_NAME);
        let _ = fs::create_dir_all(&shared_target_dir);

        // Build esp-rom-sys from repo root to ensure native blob is available
        // Build esp-rom-sys by building the esp32c6 example (preferred) so Cargo
        // will execute the transitive build script that produces the native blob.
        // Fall back to `-p esp-rom-sys` if the example isn't present.
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let esp_example_manifest = repo_root
            .join("examples")
            .join("esp32c6")
            .join("Cargo.toml");

        if esp_example_manifest.exists() {
            let _ = Command::new("cargo")
                .env("CARGO_TARGET_DIR", &shared_target_dir)
                .args(&[
                    "build",
                    "--manifest-path",
                    esp_example_manifest.to_str().unwrap(),
                    "--color=never",
                    "--target",
                    "riscv32imac-unknown-none-elf",
                ])
                .current_dir(repo_root)
                .output();
        } else {
            let _ = Command::new("cargo")
                .current_dir(repo_root)
                .env("CARGO_TARGET_DIR", &shared_target_dir)
                .args(&[
                    "build",
                    "-p",
                    "esp-rom-sys",
                    "--color=never",
                    "--target",
                    "riscv32imac-unknown-none-elf",
                ])
                .output();
        }
    });
}

#[allow(dead_code)]
fn run_test(test_file: &Path, should_pass: bool) {
    let test_name = test_file
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Failed to get file stem")
        .to_string();
    let temp_dir = Path::new(env!("OUT_DIR")).join("cases").join(&test_name);
    fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory for test");

    // Create a cargo workspace, to have one shared target directory for all tests
    let workspace_manifest_path = Path::new(env!("OUT_DIR")).join("Cargo.toml");
    if !workspace_manifest_path.exists() {
        let m = WorkspaceManifest::new();
        m.write(&workspace_manifest_path)
            .expect("Failed to create workspace manifest");
    }

    let mut manifest = Manifest::from_rust_file(test_file).unwrap();
    manifest.fix(test_file).unwrap();
    let manifest_path = temp_dir.join("Cargo.toml");
    manifest.write(&manifest_path).unwrap();
    write_lockfile_if_needed(&manifest, &manifest_path).unwrap();

    //TODO: only link embedded-test.x for some tests, to check for errors if missing

    let mut build_rs = build_rs::BuildRs::new();
    if manifest.dependencies.contains_key("esp-hal") {
        build_rs.add_linker_file("linkall.x");
    }
    build_rs.add_linker_file("embedded-test.x");

    let build_rs_path = temp_dir.join("build.rs");
    build_rs.write(&build_rs_path).unwrap();

    pre_build_esp_rom_sys();

    let cargo_target_dir = test_target_dir(&test_name);
    fs::create_dir_all(&cargo_target_dir).unwrap();

    // Copy esp-rom-sys pre-built artifacts to per-test target dir
    copy_prebuilt_esp_rom_sys_artifacts(&cargo_target_dir);

    let output = Command::new("cargo")
        .env("CARGO_TARGET_DIR", &cargo_target_dir)
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
    println!(
        "Running test {} in {}",
        test_file.display(),
        manifest_path.display()
    );

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
            let actual_stderr = normalize_line_endings(&combined_output(&output));
            match (expected_stderr, actual_stderr) {
                (Some(expected), actual) if actual.contains(&normalize_line_endings(&expected)) => {
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
