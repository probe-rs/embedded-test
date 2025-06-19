use core::convert::Infallible;
use std::sync::LazyLock;

pub use linkme;
pub use linkme::distributed_slice;

#[derive(Debug, serde::Serialize)]
pub struct Tests<'a> {
    pub version: u32,
    pub tests: &'a [Test],
}

#[derive(Debug, serde::Serialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Test {
    #[serde(serialize_with = "ser_test_name")]
    pub name: &'static str,
    #[serde(skip)]
    pub function: fn() -> !,
    pub should_panic: bool,
    pub ignored: bool,
    pub timeout: Option<u32>,
}

fn strip_crate_name(name: &str) -> Option<&str> {
    let col = name.find("::")?;
    Some(&name[col + 2..])
}
fn ser_test_name<S: serde::Serializer>(name: &'static str, s: S) -> Result<S::Ok, S::Error> {
    let name = strip_crate_name(name).ok_or(serde::ser::Error::custom("test name has no ::"))?;
    s.serialize_str(name)
}

#[distributed_slice]
pub static TESTS: [Test];

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

pub fn run_test(test_name: &str) -> ! {
    let test = TESTS
        .iter()
        .find(|t| strip_crate_name(t.name) == Some(test_name));
    if let Some(test) = test {
        (test.function)();
    } else {
        panic!("Test '{}' not found", test_name);
    }
}

pub fn print_test_list() -> ! {
    let tests = Tests {
        version: 0, // Old version. New version signals tests via ELF directly
        tests: &TESTS,
    };

    let tests_json = serde_json::to_string(&tests).expect("conversion to json");
    println!("{tests_json}");
    std::process::exit(0);
}
