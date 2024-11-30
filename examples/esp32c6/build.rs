use std::env;

fn main() {
    // esp-hal specific
    println!("cargo:rustc-link-arg=-Tlinkall.x");

    // add linker script for embedded-test!!
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");

    // Check if the `defmt` feature is enabled, and if so link its linker script
    if env::var("CARGO_FEATURE_DEFMT").is_ok() {
        println!("cargo:rustc-link-arg=-Tdefmt.x");
    }
}
