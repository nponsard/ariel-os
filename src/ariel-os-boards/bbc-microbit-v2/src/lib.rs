#![no_std]

use ariel_os_debug::log::debug;

pub fn init() {
    debug!("bbc_microbit_v2::init()");
    nrf52::init();
}
