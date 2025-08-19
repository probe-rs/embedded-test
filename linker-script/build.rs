use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let out = &PathBuf::from(env::var("OUT_DIR")?);

    let mut linker_script_contents = fs::read_to_string("embedded-test.x")?;

    // Add INSERT directive to the linker script if the `std` feature is enabled.
    if env::var("CARGO_FEATURE_STD").is_ok() {
        // Without this `INSERT ...`, including this linker script using `-T...` will
        // replace the default linker script if this is the first linker script included.
        // As this hack causes problems on xtensa, we enable it only when the `std` feature is enabled.
        // See https://github.com/probe-rs/embedded-test/pull/67 for more information.

        linker_script_contents += "\nINSERT AFTER .comment;\n"
    }

    let linker_script = out.join("embedded-test.x");
    fs::write(linker_script, linker_script_contents)?;

    println!("cargo:rustc-link-search={}", out.display());

    Ok(())
}
