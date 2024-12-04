#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests]
mod test {


    #[init]
    fn init() -> () {
        let _p = embassy_stm32::init(Default::default());
    }

    #[test]
    fn test() {
        rtt_target::rtt_init_log!();
        log::error!("Hello, log!");


        assert_eq!(1 + 1, 2);
    }
}
