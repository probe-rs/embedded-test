use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let out = &PathBuf::from(env::var("OUT_DIR")?);
    let linker_script = fs::read_to_string("embedded-test.x")?;
    fs::write(out.join("embedded-test.x"), linker_script)?;
    println!("cargo:rustc-link-search={}", out.display());

    Ok(())
}
