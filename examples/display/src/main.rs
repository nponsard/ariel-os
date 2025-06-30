#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    gpio::{Level, Output},
    time::Timer,
};

use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::image::Image;
use embedded_graphics::prelude::Point;
use embedded_graphics::{pixelcolor::Rgb565, prelude::RgbColor};
use mipidsi::{
    Builder,
    interface::Generic16BitBus,
    models::{ILI9341Rgb666, ST7789},
    options::{Orientation, Rotation},
};
use mipidsi::{interface::ParallelInterface, options::ColorOrder};
use tinybmp::Bmp;
#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: pins::LcdPeripherals) {
    // unsafe{

    //     core::arch::asm!(
    //         "svc 36",
    //     );
    // }

    let mut fsmc_d0 = Output::new(peripherals.fsmc_d0, Level::Low);
    let mut fsmc_d1 = Output::new(peripherals.fsmc_d1, Level::Low);
    let mut fsmc_d2 = Output::new(peripherals.fsmc_d2, Level::Low);
    let mut fsmc_d3 = Output::new(peripherals.fsmc_d3, Level::Low);
    let mut fsmc_d4 = Output::new(peripherals.fsmc_d4, Level::Low);
    let mut fsmc_d5 = Output::new(peripherals.fsmc_d5, Level::Low);
    let mut fsmc_d6 = Output::new(peripherals.fsmc_d6, Level::Low);
    let mut fsmc_d7 = Output::new(peripherals.fsmc_d7, Level::Low);
    let mut fsmc_d8 = Output::new(peripherals.fsmc_d8, Level::Low);
    let mut fsmc_d9 = Output::new(peripherals.fsmc_d9, Level::Low);
    let mut fsmc_d10 = Output::new(peripherals.fsmc_d10, Level::Low);
    let mut fsmc_d11 = Output::new(peripherals.fsmc_d11, Level::Low);
    let mut fsmc_d12 = Output::new(peripherals.fsmc_d12, Level::Low);
    let mut fsmc_d13 = Output::new(peripherals.fsmc_d13, Level::Low);
    let mut fsmc_d14 = Output::new(peripherals.fsmc_d14, Level::Low);
    let mut fsmc_d15 = Output::new(peripherals.fsmc_d15, Level::Low);

    let mut fsmc_noe = Output::new(peripherals.fsmc_noe, Level::High);
    let mut fsmc_nwe = Output::new(peripherals.fsmc_nwe, Level::Low);
    let mut fsmc_ne1 = Output::new(peripherals.fsmc_ne1, Level::Low);

    let mut fsmc_a16 = Output::new(peripherals.fsmc_a16, Level::Low);

    let bus = Generic16BitBus::new((
        fsmc_d0, fsmc_d1, fsmc_d2, fsmc_d3, fsmc_d4, fsmc_d5, fsmc_d6, fsmc_d7, fsmc_d8, fsmc_d9,
        fsmc_d10, fsmc_d11, fsmc_d12, fsmc_d13, fsmc_d14, fsmc_d15,
    ));

    let mut lcd = Output::new(peripherals.lcd_pow_en, Level::High);
    let mut lcd_reset = Output::new(peripherals.lcd_reset, Level::High);
    let mut lcd_ext = Output::new(peripherals.lcd_ext, Level::High);
    let mut led_blue = Output::new(peripherals.led_blue, Level::Low);
    // let mut lcd_csx = Output::new(peripherals.lcd_csx, Level::High);
    let mut led = Output::new(peripherals.lcd_light, Level::High);

    let di = ParallelInterface::new(bus, fsmc_a16, fsmc_nwe);
    // lcd.set_high();
    // led.set_high();
    let orientation = Orientation::new().rotate(Rotation::Deg270);
    let bmp_data = include_bytes!("../static/HexaCube.bmp");
    let bmp = Bmp::from_slice(bmp_data).unwrap();
    let mut display = Builder::new(ST7789, di)
        .reset_pin(lcd_reset)
        .orientation(orientation)
        .color_order(ColorOrder::Rgb)
        .init(&mut ariel_os::time::Delay)
        .unwrap();

    loop {
        display.clear(Rgb565::RED).unwrap();
        Image::new(&bmp, Point::new(54, 0))
            .draw(&mut display)
            .unwrap();

        led_blue.toggle();
        Timer::after_millis(2000).await;
        led_blue.toggle();

        display.clear(Rgb565::BLUE).unwrap();
        Image::new(&bmp, Point::new(54, 0))
            .draw(&mut display)
            .unwrap();

        Timer::after_millis(2000).await;
    }
}
