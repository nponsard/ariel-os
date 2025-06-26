#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::*};
use nrf_modem::ConnectionPreference;
use nrf_modem::SystemMode;
#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello World!");

    let a = nrf_modem::init(SystemMode {
        lte_support: true,
        lte_psm_support: true,
        nbiot_support: true,
        gnss_support: true,
        preference: ConnectionPreference::None,
    })
    .await;

    if a.is_err(){
        error!("Failed to initialize modem: {:?}", a);
        exit(ExitCode::FAILURE);
    }
    info!("Modem initialized");


    let response = nrf_modem::send_at::<64>("AT+CGMI").await.unwrap();
    info!("Modem Manufacturer: {}", response.as_str());
}
