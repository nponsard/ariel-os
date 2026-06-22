#![no_main]
#![no_std]

mod hid;

use embassy_futures::join::join;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Duration;
use trouble_host::{
    BleHostError, Controller, Error,
    advertise::{
        AdStructure, Advertisement, AdvertisementParameters, BR_EDR_NOT_SUPPORTED,
        LE_GENERAL_DISCOVERABLE,
    },
    gap::{GapConfig, PeripheralConfig},
    gatt::{GattConnection, GattConnectionEvent, GattEvent},
    prelude::{
        DefaultPacketPool, EventHandler, FromGatt, Peripheral, PhyKind, TxPower, appearance,
        gatt_server, gatt_service, service,
    },
};
use usbd_hid::descriptor::{AsInputReport, SerializedDescriptor};

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    log::{Debug2Format, error, info},
    reexports::embassy_time,
};
use ariel_os_boards::pins;

use crate::hid::KeypadReport;

const NAME: &str = "Ariel OS BLE keyboard";

static KEYS_CHANNEL: Channel<CriticalSectionRawMutex, [u8; 6], 10> = Channel::new();
static LEDS_CHANNEL: Channel<CriticalSectionRawMutex, u8, 10> = Channel::new();

// GATT Server definition
#[gatt_server]
struct Server {
    hid_service: HidService,
}

#[gatt_service(uuid = service::HUMAN_INTERFACE_DEVICE)]
pub(crate) struct HidService {
    #[characteristic(uuid = "2a4a", read, value = [0x01, 0x01, 0x00, 0x03])]
    pub(crate) hid_info: [u8; 4],

    // info!("len: {}", KeypadReport::desc().len());
    #[characteristic(uuid = "2a4b", read, value = KeypadReport::desc().try_into().expect("converting hid report to an [u8; 42] (check if size is correct)"))]
    pub(crate) report_map: [u8; 42],
    #[characteristic(uuid = "2a4c", write_without_response)]
    pub(crate) hid_control_point: u8,
    #[characteristic(uuid = "2a4e", read, write_without_response, value = 1)]
    pub(crate) protocol_mode: u8,
    #[descriptor(uuid = "2908", read, value = [0u8, 1u8])]
    #[characteristic(uuid = "2a4d", read, notify)]
    pub(crate) input_keyboard: [u8; 8],
    #[descriptor(uuid = "2908", read, value = [0u8, 2u8])]
    #[characteristic(uuid = "2a4d", read, write, write_without_response)]
    pub(crate) output_keyboard: [u8; 1],
}

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    info!("starting ble stack");
    let stack = ariel_os::ble::ble_stack().await;
    let mut host = stack.build();

    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: NAME,
        appearance: &appearance::human_interface_device::KEYBOARD,
    }))
    .unwrap();

    info!("Using address: {}", ariel_os::ble::current_address().await);

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
    let _ = join(host.runner.run(), async {
        // let mut _session = scanner.scan(&config).await.unwrap();
        loop {
            match advertise(NAME, &mut host.peripheral, &server).await {
                Ok(conn) => {
                    let keypad = async {
                        loop {
                            let keycodes = KEYS_CHANNEL.receive().await;
                            let mut buf = [0u8; 8];

                            let report = KeypadReport {
                                keycodes,
                                ..Default::default()
                            };
                            let n = report.serialize(&mut buf).unwrap();

                            server
                                .hid_service
                                .input_keyboard
                                .notify(&conn, &buf)
                                .await
                                .map_err(|e| error!("Failed to notify HID report: {:?}", e))
                                .unwrap();
                        }
                    };
                    // set up tasks when the connection is established to a central, so they don't run when no one is connected.
                    let res =
                        embassy_futures::join::join(gatt_events_task(&server, &conn), keypad).await;

                    info!("res : {:?}", Debug2Format(&res));
                }
                Err(e) => {
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    })
    .await;
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, 'b, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C, DefaultPacketPool>,
    server: &'b Server<'_>,
) -> Result<GattConnection<'a, 'b, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 60];
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[service::HUMAN_INTERFACE_DEVICE.to_le_bytes()]),
            AdStructure::CompleteLocalName(name.as_bytes()),
            AdStructure::Unknown {
                ty: 0x19, // Appearance
                data: &appearance::human_interface_device::KEYBOARD.to_le_bytes(),
            },
        ],
        &mut advertiser_data[..],
    )?;

    let advertise_config = AdvertisementParameters::default();

    let advertiser = peripheral
        .advertise(
            &advertise_config,
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..],
                scan_data: &[],
            },
        )
        .await?;
    info!("advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("connection established");
    Ok(conn)
}

/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_task(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, DefaultPacketPool>,
) -> Result<(), Error> {
    loop {
        match conn.next().await {
            // TODO : handle security
            GattConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => {
                let payload = event.payload();

                let incoming = payload.incoming();
                info!("Gatt incoming: {:?}", incoming);

                match event {
                    GattEvent::Write(event) => {
                        if event.handle() == server.hid_service.output_keyboard.handle {
                            let data = event.data();

                            if let Some(d) = data.get(0) {
                                // Don't block if channel is full
                                let _ = LEDS_CHANNEL.try_send(*d);
                            }
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
    info!("[gatt] task finished");
    Ok(())
}

#[ariel_os::task(autostart, peripherals)]
async fn button(peripherals: pins::ButtonPeripherals) {
    let mut btn0 = Input::builder(peripherals.button0, Pull::Up)
        .build_with_interrupt()
        .unwrap();

    loop {
        btn0.wait_for_any_edge().await;
        // Which keys are currently pressed, keycodes available here (section 10): https://www.usb.org/sites/default/files/hut1_7.pdf
        let mut keys = [0u8; 6];
        if btn0.get_level() == Level::High {
            keys[0] = 0x04;
        }
        KEYS_CHANNEL.send(keys).await;
    }
}

#[ariel_os::task(autostart, peripherals)]
async fn led(peripherals: pins::LedPeripherals) {
    let mut led0 = Output::new(peripherals.led0, Level::Low);

    loop {
        let led_status = LEDS_CHANNEL.receive().await;

        if led_status & 0x02 == 0x02 {
            led0.set_high();
        } else {
            led0.set_low();
        }
    }
}
