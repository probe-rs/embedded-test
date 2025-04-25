use core::convert::Infallible;
use std::sync::LazyLock;

use crate::export::Tests;

type Args = Vec<Result<&'static str, Infallible>>;

pub fn args() -> Result<Args, Infallible> {
    // We want to return an Iterator over `Result<&'static str, Error>` in order to
    // match the API of semihosting's `args()` / `Args`.
    // `std` has `String` args, so let's make those `'static`.
    // We also skip `argv[0]`, which on `std` contains the binary name.
    static ARGS: LazyLock<Vec<String>> = LazyLock::new(|| std::env::args().skip(1).collect());

    Ok(ARGS
        .iter()
        .map(|s| Ok(s.as_str()))
        .collect::<Vec<Result<&'static str, Infallible>>>())
}

pub fn abort() -> ! {
    std::process::abort()
}

pub fn exit(code: i32) -> ! {
    std::process::exit(code)
}

pub fn print_test_list<const JSON_SIZE_TOTAL: usize>(tests: &Tests<'_>) {
    let tests_json =
        serde_json_core::to_string::<_, JSON_SIZE_TOTAL>(tests).expect("conversion to json");

    println!("{tests_json}");
}
