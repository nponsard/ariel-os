#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::*};
use ariel_os::time::Timer;
use core::arch::asm;
use nrf_modem::ConnectionPreference;
use nrf_modem::SystemMode;
#[ariel_os::task(autostart)]
async fn main() {
    // info!("waiting");
    // Timer::after_secs(10).await;

    info!("Hello World!");

    // cortex_m::asm::delay(600_000_000);

    let a = nrf_modem::init(SystemMode {
        lte_support: true,
        lte_psm_support: false,
        nbiot_support: false,
        gnss_support: false,
        preference: ConnectionPreference::None,
    })
    .await
    .unwrap();

    info!("Modem initialized");

    let response = nrf_modem::send_at::<64>("AT+CGMI").await.unwrap();
    info!("Modem Manufacturer: {}", response.as_str());
}
