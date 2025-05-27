#![no_main]
#![no_std]

use ariel_os::asynch::Spawner;
use ariel_os::debug::log::info;
use bt_hci::cmd::le::*;
use bt_hci::controller::ControllerCmdSync;
use embassy_futures::join::join;
use embassy_time::{Duration, Timer};

use trouble_host::prelude::*;

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    let spawner = Spawner::for_current_executor().await;
    // info!("starting ble stack");
    // ariel_os::ble::run_example(spawner).await;

    // let mut adv_data = [0; 31];
    // AdStructure::encode_slice(
    //     &[
    //         AdStructure::CompleteLocalName(b"Trouble"),
    //         AdStructure::ServiceUuids16(&[[0x0f, 0x18]]),
    //         AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
    //     ],
    //     &mut adv_data[..],
    // )
    // .unwrap();

    // let mut scan_data = [0; 31];
    // AdStructure::encode_slice(
    //     &[AdStructure::CompleteLocalName(b"Trouble")],
    //     &mut scan_data[..],
    // )
    // .unwrap();

    // info!("Starting advertising");

    // // stack.runner.run().await.unwrap();

    // let _ = join(stack.runner.run(), async {
    //     loop {
    //         let mut params = AdvertisementParameters::default();

    //         // params.interval_min = Duration::from_millis(100);

    //         // params.interval_max = Duration::from_millis(100);

    //         let advertiser = stack
    //             .peripheral
    //             .advertise(
    //                 &params,
    //                 Advertisement::ConnectableScannableUndirected {
    //                     adv_data: &adv_data[..],
    //                     scan_data: &scan_data[..],
    //                 },
    //             )
    //             .await
    //             .unwrap();
    //         info!("Advertising started");
    //         let conn = advertiser.accept().await.unwrap();

    //         info!("Connection established");

    //         // let mut ch1 = L2capChannel::accept(&stack, &conn, &[0x2349], &Default::default())
    //         //     .await
    //         //     .unwrap();

    //         info!("L2CAP channel accepted");

    //         // Size of payload we're expecting
    //         const PAYLOAD_LEN: usize = 27;
    //         let mut rx = [0; PAYLOAD_LEN];
    //         // for i in 0..10 {
    //         //     let len = ch1.receive(&stack, &mut rx).await.unwrap();
    //         //     assert_eq!(len, rx.len());
    //         //     assert_eq!(rx, [i; PAYLOAD_LEN]);
    //         // }

    //         info!("L2CAP data received, echoing");
    //         Timer::after(Duration::from_secs(1)).await;
    //         // for i in 0..10 {
    //         //     let tx = [i; PAYLOAD_LEN];
    //         //     ch1.send::<_, L2CAP_MTU>(&stack, &tx).await.unwrap();
    //         // }
    //         info!("L2CAP data echoed");

    //         Timer::after(Duration::from_secs(60)).await;
    //     }
    // })
    // .await;
}
