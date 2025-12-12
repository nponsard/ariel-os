#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    debug::log::debug,
    gpio::{Level, Output},
    time::Timer,
};

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: pins::LedPeripherals) {
    let mut led0 = Output::new(peripherals.led0, Level::Low);

    loop {
        let runlevel = esp_hal::interrupt::current_runlevel();

        let mstatus_st = esp_hal::riscv::register::mstatus::read();

        debug!(
            "mie: {}, mpie: {}, runlevel: {:?}",
            mstatus_st.mie(),
            mstatus_st.mpie(),
            runlevel
        );
        led0.toggle();
        Timer::after_millis(500).await;
    }
}
