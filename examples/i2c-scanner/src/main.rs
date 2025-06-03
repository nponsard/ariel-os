#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::log::info,
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
};

use embedded_hal_async::i2c::I2c;

#[ariel_os::task(autostart, peripherals)]
async fn i2c_scanner(peripherals: pins::Peripherals) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };

    let mut i2c_bus = pins::SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);

    info!("Checking for I2C devices on the bus...");

    for addr in 1..=127 {
        if i2c_bus.write(addr, &[]).await.is_ok() {
            info!("Found device at address 0x{:x}", addr);
        }
    }

    info!("Done checking. Have a great day!");
}
