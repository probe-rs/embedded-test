use std::{env, fs::File, io::Write, path::PathBuf};

#[cfg(feature = "nrf51422")]
const MEM_X: &[u8] = include_bytes!("memory-nrf51.x");
#[cfg(feature = "nrf52832")]
const MEM_X: &[u8] = include_bytes!("memory-nrf52.x");
#[cfg(feature = "nrf52833")]
const MEM_X: &[u8] = include_bytes!("memory-nrf52833.x");

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(MEM_X)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
}
