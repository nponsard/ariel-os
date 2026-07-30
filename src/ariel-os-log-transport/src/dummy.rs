use embassy_executor::Spawner;

use ariel_os_hal::hal::OptionalPeripherals;

pub(crate) fn write_bytes(_bytes: &[u8]) {}

pub(crate) fn flush() {}

/// Initialize the transport driver.
pub fn init(_peripherals: &mut OptionalPeripherals, _spawner: Spawner) {}
