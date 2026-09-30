#![allow(missing_docs)]

use ariel_os_embassy_common::bootloader::{
    BootLoaderBackend, BootloaderPartitions, BootloaderStorage, FlashConfig,
};
use embassy_boot::BootLoaderConfig;
use embedded_storage::nor_flash::{self, NorFlash, ReadNorFlash};

use crate::hal::OptionalPeripherals;

pub fn init(peripherals: &mut OptionalPeripherals) {}

// TODO: dummy updater interface
// TODO: additional feature for BootloaderStorage implementation ?

pub struct HalBootLoaderBackend;

impl BootloaderStorage for HalBootLoaderBackend {
    type ACTIVE = DummyFlash;
    type DFU = DummyFlash;
    type STATE = DummyFlash;

    fn partitions(
        flash_config: &ariel_os_embassy_common::bootloader::FlashConfig,
    ) -> BootloaderPartitions<Self::ACTIVE, Self::DFU, Self::STATE> {
        unimplemented!()
    }
}

#[cfg(feature = "boot-loader")]
impl BootLoaderBackend for HalBootLoaderBackend {
    fn load_active(flash_config: &ariel_os_embassy_common::bootloader::FlashConfig) {
        unimplemented!()
    }
    fn enable_watchdog() {
        unimplemented!()
    }
    fn pet_watchdog() {
        unimplemented!()
    }
}

// Dummy NorFlash implementation.

pub struct DummyFlash;
impl NorFlash for DummyFlash {
    const ERASE_SIZE: usize = 42;
    const WRITE_SIZE: usize = 42;
    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        unimplemented!()
    }
    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl ReadNorFlash for DummyFlash {
    const READ_SIZE: usize = 42;
    fn capacity(&self) -> usize {
        unimplemented!()
    }
    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl nor_flash::ErrorType for DummyFlash {
    type Error = FlashError;
}

#[derive(Debug, PartialEq)]
pub struct FlashError;
impl nor_flash::NorFlashError for FlashError {
    fn kind(&self) -> nor_flash::NorFlashErrorKind {
        unimplemented!()
    }
}
