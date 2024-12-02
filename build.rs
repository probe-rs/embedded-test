use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// Macros taken from:
// https://github.com/TheDan64/inkwell/blob/36c3b10/src/lib.rs#L81-L110

// Given some features, assert that AT MOST one of the features is enabled.
macro_rules! assert_unique_features {
    () => {};

    ( $first:tt $(,$rest:tt)* ) => {
        $(
            #[cfg(all(feature = $first, feature = $rest))]
            compile_error!(concat!("Features \"", $first, "\" and \"", $rest, "\" cannot be used together"));
        )*
        assert_unique_features!($($rest),*);
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_unique_features!("log", "defmt");

    let out = &PathBuf::from(env::var("OUT_DIR")?);
    let linker_script = fs::read_to_string("embedded-test.x")?;
    fs::write(out.join("embedded-test.x"), linker_script)?;
    println!("cargo:rustc-link-search={}", out.display());

    Ok(())
}
