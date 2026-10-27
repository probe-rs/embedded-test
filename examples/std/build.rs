use std::env;

fn main() {
    // Note: no linker script is needed on std.
    println!("cargo::rustc-check-cfg=cfg(rust_analyzer)");

    // Check if the `defmt` feature is enabled, and if so link its linker script
    if env::var("CARGO_FEATURE_DEFMT").is_ok() {
        panic!("defmt not supported on std!");
    }
}
