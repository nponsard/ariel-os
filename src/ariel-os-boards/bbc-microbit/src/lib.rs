#![no_std]

use ariel_os_debug::log::debug;

pub fn init() {
    debug!("bbc_microbit_::init()");
    nrf51::init();
}
