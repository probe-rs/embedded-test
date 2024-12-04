fn main() {
    println!("cargo::rustc-link-arg=-Tlinkall.x");
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
}
