fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo::rustc-link-arg=-Tlink.x");
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
}
