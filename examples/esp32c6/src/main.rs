#![no_std]
#![no_main]

esp_bootloader_esp_idf::esp_app_desc!();

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use log::info;

/// ====> Look in the tests directory to see how embedded-test works <====

/// This file here is just a simple async main function that prints "Hello world!" every second,
/// similar to the example generate by esp-generate.

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    esp_println::logger::init_logger_from_env(); // # prints to jtag/uart0 !

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}
