#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::prelude::*;
use log::info;

/// ====> Look in the tests directory to see how embedded-test works <====

/// This file here is just a simple async main function that prints "Hello world!" every second,
/// similar to the example generate by esp-generate.

#[main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env(); // # prints to jtag/uart0 !

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER)
        .split::<esp_hal::timer::systimer::Target>();
    esp_hal_embassy::init(timer0.alarm0);

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}
