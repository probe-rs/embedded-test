use std::env;

fn main() {
    // add linker script for embedded-test!!
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");

    // Check if the `defmt` feature is enabled, and if so link its linker script
    if env::var("CARGO_FEATURE_DEFMT").is_ok() {
        panic!("defmt not supported on std!");
    }
}
