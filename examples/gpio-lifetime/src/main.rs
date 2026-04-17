#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    log::info,
    time::Timer,
};

#[ariel_os::task(autostart, peripherals)]
async fn blinky(mut peripherals: pins::LedPeripherals) {
    // First use as input
    // esp-hal lets us have both `input` and `led` in the same scope but embassy wants us to drop `btn` before using `peripherals.led0` again.
    {
        let input = Input::builder(peripherals.led0.reborrow(), Pull::Down).build();

        let level = input.get_level();
        info!("level: {:?}", level);
    }

    // Then blink
    let mut led = Output::new(peripherals.led0, Level::Low);

    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
