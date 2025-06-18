mod attributes;

// Copied from https://github.com/knurling-rs/defmt/blob/main/firmware/defmt-test/macros/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;

/// Attribute to be placed on the test suite's module.
///
/// ## Arguments
/// - `default-timeout`: The default timeout in seconds for all tests in the suite. This can be overridden on a per-test basis. If not specified here or on a per-test basis, the default timeout is 60 seconds.
/// - `executor`: The custom executor to use for running async tests. This is only required if the features `embassy` and `external-executor` are enabled.
/// - `setup`: A function that will be called before running the tests. This can be used to setup logging or other global state.
///
/// ## Examples
///
/// Define a test suite with a single test:
///
/// ```rust,no_run
/// #[embedded_test::tests]
/// mod tests {
///     #[init]
///     fn init() {
///         // Initialize the hardware
///     }
///
///     #[test]
///     fn test() {
///        // Test the hardware
///     }
/// }
/// ```
///
/// Define a test suite and customize everything:
///
/// ```rust,no_run
/// #[embedded_test::tests(default_timeout = 10, executor = embassy::executor::Executor::new(), setup = rtt_target::rtt_init_log!())]
/// mod tests {
///     #[init]
///     fn init() {
///         // Initialize the hardware
///     }
///
///     #[test]
///     fn test() {
///         log::info("Start....")
///         // Test the hardware
///     }
///
///     #[test]
///     #[timeout(5)]
///     fn test2() {
///        // Test the hardware
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn tests(args: TokenStream, input: TokenStream) -> TokenStream {
    attributes::tests::expand(args, input).unwrap_or_else(|e| e.to_compile_error().into())
}
