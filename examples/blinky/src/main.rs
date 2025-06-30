#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    gpio::{Level, Output},
    time::Timer,
};

use mipidsi::models::ILI9341Rgb666;
use mipidsi::options::ColorOrder;

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: pins::LedPeripherals) {
    // unsafe{

    //     core::arch::asm!(
    //         "svc 36",
    //     );
    // }

    let mut led = Output::new(peripherals.lcd_light, Level::Low);
    let mut lcd = Output::new(peripherals.lcd_pow_en, Level::Low);

    lcd.set_high();


    // The micro:bit uses an LED matrix; pull the column line low.
    #[cfg(any(context = "bbc-microbit-v2", context = "bbc-microbit-v1"))]
    let _led_col1 = Output::new(peripherals.led_col1, Level::Low);

    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
