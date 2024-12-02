#![no_main]
#![no_std]

/// ====> Look in the tests directory to see how embedded-test works <====
/// This file here is just a simple blinky example, as found in the examples of the stm32f7xx-hal crate.
use core::panic::PanicInfo;
use stm32f7xx_hal::gpio::GpioExt;
use stm32f7xx_hal::pac;
use stm32f7xx_hal::prelude::*;

#[no_mangle]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-144 it's connected to pin PB7.
        let gpiob = dp.GPIOB.split();
        let mut led = gpiob.pb7.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // On for 1s, off for 1s.
            led.set_high();
            delay.delay_ms(1000_u32);
            led.set_low();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
