/*
```cargo
[dependencies]
embedded-test = { path = "../../../.." }
rtt-target = { version = "0.6.1", features = ["log"] }
log = { version = "0.4.20" }
esp-hal = { version = "0.23.1", features = ["esp32c6"] } # for critical section implementation
[lib]
harness = false
```
 */

#![no_std]
#![no_main]

#[cfg(test)]
#[export_name = "baaad"]
#[embedded_test::setup]
fn setup() {
    rtt_target::rtt_init_log!();
}

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use esp_hal::*; // needs to be in scope, to prevent linker error about missing `critical_section` implementation
    #[test]
    fn ok() {
        log::info!("Hello world");
    }
}
