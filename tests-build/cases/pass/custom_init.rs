/*
```cargo
[dependencies]
embassy-executor = { version = "0.8", features = ["executor-thread", "arch-riscv32"] }
esp-hal = { version = "=1.0.0-rc.0", features = ["esp32c6"] } # for critical section implementation
embedded-test = { path = "../../..", features = ["embassy"] }

[lib]
harness = false
```
 */

#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use esp_hal::*; // needs to be in scope, to prevent linker error about missing `critical_section` implementation

    struct Context;

    #[init]
    async fn def_init() -> Context {
        Context
    }

    #[test]
    async fn takes_context(_state: Context) {
        assert!(true)
    }

    #[test]
    fn takes_context2(_state: Context) {
        assert!(true)
    }

    #[test(init=def_init)]
    fn takes_context3(_state: Context) {
        assert!(true)
    }

    #[test(init=def_init)]
    async fn takes_context4(_state: Context) {
        assert!(true)
    }

    #[test]
    async fn takes_no_context() {
        assert!(true)
    }

    #[test(init = def_init)]
    async fn takes_no_context2() {
        assert!(true)
    }

    #[test]
    fn takes_no_context3() {
        assert!(true)
    }

    #[test]
    async fn takes_no_context4() {
        assert!(true)
    }

    #[test(init = def_init)]
    fn takes_no_context5() {
        assert!(true)
    }

    #[test(init = def_init)]
    async fn takes_no_context6() {
        assert!(true)
    }

    async fn alt_init() -> (Context, u32) {
        (Context, 42)
    }

    #[test(init = alt_init)]
    async fn takes_alt(_state: (Context, u32)) {
        assert!(true)
    }

    #[test(init = alt_init)]
    fn takes_alt2(_state: (Context, u32)) {
        assert!(true)
    }

    #[test(init = def_init)]
    async fn takes_no_alt() {
        assert!(true)
    }

    #[test(init = def_init)]
    fn takes_no_alt2() {
        assert!(true)
    }

    fn sync_init() -> f32 {
        3.14
    }

    #[test(init = sync_init)]
    async fn takes_sync(_state: f32) {
        assert!(true)
    }

    #[test(init = sync_init)]
    fn takes_sync_2(_state: f32) {
        assert!(true)
    }

    #[test(init = sync_init)]
    async fn takes_no_sync() {
        assert!(true)
    }

    #[test(init = sync_init)]
    fn takes_no_sync2() {
        assert!(true)
    }
}
