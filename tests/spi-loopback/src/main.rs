//! This example is merely to illustrate and test raw bus usage.
//!
//! Please use [`ariel_os::sensors`] instead for a high-level sensor abstraction that is
//! HAL-agnostic.
#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::{ExitCode, exit},
    gpio, hal,
    log::{Hex, debug, info},
    spi::{
        Mode,
        main::{Kilohertz, SpiDevice, highest_freq_in},
    },
};
use embassy_sync::mutex::Mutex;
use embedded_hal_async::spi::SpiDevice as _;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    let mut spi_config = hal::spi::main::Config::default();
    spi_config.frequency = const { highest_freq_in(Kilohertz::kHz(1000)..=Kilohertz::kHz(2000)) };
    let mut spi_config2 = hal::spi::main::Config::default();
    spi_config2.frequency = const { highest_freq_in(Kilohertz::kHz(1000)..=Kilohertz::kHz(2000)) };
    debug!("Selected frequency: {:?}", spi_config.frequency);
    spi_config.mode = if !cfg!(context = "esp") {
        Mode::Mode3
    } else {
        // FIXME: the sensor datasheet does say SPI mode 3, not mode 0
        Mode::Mode0
    };

    let spi_bus = pins::SensorSpi::new(
        peripherals.spi_sck,
        peripherals.spi_miso,
        peripherals.spi_mosi,
        spi_config,
    );
    let spi_bus = Mutex::new(spi_bus);


    let spi_bus2 = pins::SensorSpi::new(
        peripherals.spi_sck2,
        peripherals.spi_miso2,
        peripherals.spi_mosi2,
        spi_config2,
    );
    let spi_bus2 = Mutex::new(spi_bus2);

    let cs_output = gpio::Output::new(peripherals.spi_cs, gpio::Level::High);
    let cs_output2 = gpio::Output::new(peripherals.spi_cs2, gpio::Level::High);

    let mut spi_device = SpiDevice::new(&spi_bus, cs_output);

    let mut spi_device2 = SpiDevice::new(&spi_bus2, cs_output2);



    let out = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut in_ = [0u8; 8];
    spi_device.transfer(&mut in_, &out).await.unwrap();

    let out2 = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut in2_ = [0u8; 8];
    spi_device2.transfer(&mut in2_, &out2).await.unwrap();

    info!("got {}", Hex(in_));
    info!("got {}", Hex(in2_));


    assert_eq!(out, in_);
    assert_eq!(out2, in2_);


    info!("Test passed!");

    exit(ExitCode::SUCCESS);
}
