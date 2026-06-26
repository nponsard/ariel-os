//! Adapted from the example in `trouble_host`.
#![no_main]
#![no_std]

use ariel_os::{log::info, reexports::embassy_time, time::Timer};
use embassy_futures::join::join;
use embassy_time::Duration;
use heapless::Vec;
use trouble_host::advertise::{
    AdStructure, Advertisement, AdvertisementParameters, BR_EDR_NOT_SUPPORTED,
    LE_GENERAL_DISCOVERABLE,
};

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    info!("starting ble stack");
    let stack = ariel_os::ble::ble_stack().await;
    let mut host = stack.build();

    info!("Using address: {}", ariel_os::ble::current_address().await);

    let mut adv_data = [0; 31];

    const BEACON_TYPE: [u8; 2] = [0x02, 0x15];
    const BEACON_UUID: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    const BEACON_MAJOR: [u8; 2] = [0, 0];
    const BEACON_MINOR: [u8; 2] = [0, 0];
    const BEACON_MEASURED_POWER: [u8; 1] = [0];

    let mut manufacturer_payload: Vec<_, 27> = Vec::new();
    manufacturer_payload
        .extend_from_slice(&BEACON_TYPE)
        .unwrap();
    manufacturer_payload
        .extend_from_slice(&BEACON_UUID)
        .unwrap();
    manufacturer_payload
        .extend_from_slice(&BEACON_MAJOR)
        .unwrap();
    manufacturer_payload
        .extend_from_slice(&BEACON_MINOR)
        .unwrap();
    manufacturer_payload
        .extend_from_slice(&BEACON_MEASURED_POWER)
        .unwrap();

    let len = AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ManufacturerSpecificData {
                // Apple company identifier [0x4c, 0x00]
                company_identifier: 0x4c00,
                payload: &manufacturer_payload,
            },
        ],
        &mut adv_data[..],
    )
    .unwrap();

    info!("Starting advertising");

    let _ = join(host.runner.run(), async {
        let params = AdvertisementParameters {
            interval_min: Duration::from_millis(100),
            interval_max: Duration::from_millis(100),
            ..Default::default()
        };

        let _advertiser = host
            .peripheral
            .advertise(
                &params,
                Advertisement::NonconnectableScannableUndirected {
                    adv_data: adv_data.get(..len).unwrap(),
                    scan_data: &[],
                },
            )
            .await;

        loop {
            info!("Still running");
            Timer::after_secs(60).await;
        }
    })
    .await;
}
