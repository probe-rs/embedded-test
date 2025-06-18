#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::setup]
fn setup() {
    rtt_target::rtt_init_log!();
}

mod apple;
mod banana;
