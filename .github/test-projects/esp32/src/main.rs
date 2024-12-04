#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;

#[esp_hal::entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    loop {}
}
