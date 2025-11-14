//! This module is intended to contain the auto-@generated instantiation and registration of sensor
//! drivers.

// This example does not currently register any sensor drivers, they will be added later.
pub static NRF91_GNSS: ariel_os_nrf91_gnss::Nrf91Gnss =
    const { ariel_os_nrf91_gnss::Nrf91Gnss::new(Some("Gnss")) };
#[ariel_os::reexports::linkme::distributed_slice(ariel_os::sensors::SENSOR_REFS)]
#[linkme(crate = ariel_os::reexports::linkme)]
static NRF91_GNSS_REF: &'static dyn ariel_os::sensors::Sensor = &NRF91_GNSS;

#[ariel_os::task]
pub async fn nrf91_gnss_runner() {
    NRF91_GNSS.run().await
}
