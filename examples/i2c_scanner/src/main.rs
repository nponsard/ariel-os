#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::log::info,
    hal,
    i2c::controller::{I2cDevice, Kilohertz, highest_freq_in},
};

use embassy_sync::mutex::Mutex;
use embedded_hal_async::i2c::I2c;

pub static I2C_BUS: once_cell::sync::OnceCell<
    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::i2c::controller::I2c>,
> = once_cell::sync::OnceCell::new();

#[ariel_os::task(autostart, peripherals)]
async fn i2c_scanner(peripherals: pins::Peripherals) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };

    let i2c_bus = pins::SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);
    let _ = I2C_BUS.set(Mutex::new(i2c_bus));
    let mut i2c_device = I2cDevice::new(I2C_BUS.get().unwrap());

    info!("Checking for I2C devices on the bus...");

    for addr in 1..=127 {
        if i2c_device.write(addr, &[]).await.is_ok() {
            info!("Found device at address {}", addr);
        }
    }

    info!("Done checking. Have a great day!");
}
