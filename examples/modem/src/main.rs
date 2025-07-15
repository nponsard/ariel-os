#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::*};
use ariel_os::reexports::nrf_modem;
use ariel_os::time::Timer;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello World!");

    let response = nrf_modem::send_at::<64>("AT+CGMI").await.unwrap();
    info!("Modem Manufacturer: {}", response.as_str());
}
