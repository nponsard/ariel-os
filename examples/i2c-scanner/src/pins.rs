use ariel_os::hal::{self, i2c};

#[cfg(context = "st-nucleo-f042k6")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "st-nucleo-f042k6")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_scl: PB6,
    i2c_sda: PB7
});

#[cfg(any(context = "nrf52833", context = "nrf52840"))]
pub type SensorI2c = i2c::controller::TWISPI0;
#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: P0_16,
    i2c_scl: P0_08,
});
