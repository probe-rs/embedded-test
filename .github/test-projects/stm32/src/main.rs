#![no_std]
#![no_main]

use {panic_probe as _};

#[cortex_m_rt::entry]
fn main() -> ! {
    let _p = embassy_stm32::init(Default::default());

    loop {}
}
