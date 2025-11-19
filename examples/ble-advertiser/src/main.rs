//! Adapted from the example in `trouble_host`
#![no_main]
#![no_std]

use embassy_futures::join::join;
use trouble_host::{
    advertise::{
        AdStructure, Advertisement, AdvertisementParameters, BR_EDR_NOT_SUPPORTED,
        LE_GENERAL_DISCOVERABLE,
    },
    prelude::ConnectionEvent,
};

use ariel_os::{
    debug::log::info,
    time::{Duration, Timer},
};

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    info!("starting ble stack");
    let mut stack = ariel_os::ble::ble_stack().await.build();

    let mut adv_data = [0; 31];

    let len = AdStructure::encode_slice(
        &[
            AdStructure::CompleteLocalName(b"Ariel OS BLE"),
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
        ],
        &mut adv_data[..],
    )
    .unwrap();

    info!("Starting advertising");

    let _ = join(stack.runner.run(), async {
        let params = AdvertisementParameters {
            interval_min: Duration::from_millis(100),
            interval_max: Duration::from_millis(100),
            ..Default::default()
        };
        loop {
            let advertiser = stack
                .peripheral
                .advertise(
                    &params,
                    Advertisement::ConnectableScannableUndirected {
                        adv_data: adv_data.get(..len).unwrap(),
                        scan_data: &[],
                    },
                )
                .await
                .unwrap();
            let con = advertiser.accept().await.unwrap();
            loop {
                match con.next().await {
                    ConnectionEvent::Disconnected { reason } => {
                        info!("disconnect, reason: {:?}", reason);
                        break;
                    }
                    ConnectionEvent::ConnectionParamsUpdated {
                        conn_interval,
                        peripheral_latency,
                        supervision_timeout,
                    } => {
                        info!("ConnectionParamsUpdated");
                    }
                    ConnectionEvent::Gatt { data } => {
                        info!("Gatt");
                    }
                    ConnectionEvent::PhyUpdated { tx_phy, rx_phy } => {
                        info!("PhyUpdated")
                    }
                }
            }
        }
    })
    .await;
}
