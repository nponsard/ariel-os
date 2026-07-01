#![no_main]
#![no_std]

mod hid;

use embassy_futures::{
    join::join,
    select::{Either, select},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Duration;
use trouble_host::{
    Address, BleHostError, Controller, Error, Identity, Stack,
    advertise::{
        AdStructure, Advertisement, AdvertisementParameters, BR_EDR_NOT_SUPPORTED,
        LE_GENERAL_DISCOVERABLE,
    },
    att::AttErrorCode,
    gap::{GapConfig, PeripheralConfig},
    gatt::{GattConnection, GattConnectionEvent, GattEvent},
    prelude::{
        AddrKind, BdAddr, DefaultPacketPool, FromGatt, Peripheral, appearance, characteristic,
        descriptors, gatt_server, gatt_service, service,
    },
};
use usbd_hid::descriptor::{AsInputReport, SerializedDescriptor};

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    log::{Debug2Format, error, info},
    reexports::embassy_time,
    time::{Instant, Timer},
};
use ariel_os_boards::pins;

use crate::hid::KeypadReport;

const NAME: &str = "Ariel OS keyboard";

static KEYS_CHANNEL: Channel<CriticalSectionRawMutex, [u8; 6], 10> = Channel::new();
static LEDS_CHANNEL: Channel<CriticalSectionRawMutex, u8, 10> = Channel::new();

// GATT Server definition
#[gatt_server]
struct Server {
    battery_service: BatteryService,
    hid_service: HidService,
}

/// Battery service
#[gatt_service(uuid = service::BATTERY)]
struct BatteryService {
    /// Battery Level
    #[descriptor(uuid = descriptors::VALID_RANGE, read, value = [0, 100])]
    #[descriptor(uuid = descriptors::MEASUREMENT_DESCRIPTION, name = "hello", read, value = "Battery Level")]
    #[characteristic(uuid = characteristic::BATTERY_LEVEL, read, notify, value = 10)]
    level: u8,
}

#[gatt_service(uuid = service::HUMAN_INTERFACE_DEVICE)]
pub(crate) struct HidService {
    // bcdHID (2bytes), bCountryCode, Flags (RemoteWake)
    #[characteristic(uuid = "2a4a", read, value = [0x01, 0x11, 0x00, 0x01])]
    pub(crate) hid_info: [u8; 4],

    // info!("len: {}", KeypadReport::desc().len());
    #[characteristic(uuid = "2a4b", read, value = KeypadReport::desc().try_into().expect("converting hid report to an [u8; 67] (check if size is correct)"))]
    pub(crate) report_map: [u8; 67],

    #[characteristic(uuid = "2a4c", write_without_response)]
    pub(crate) hid_control_point: u8,
    #[characteristic(uuid = "2a4e", read, write_without_response, value = 1)]
    pub(crate) protocol_mode: u8,
    #[descriptor(uuid = "2908", read, value = [0u8, 1u8])]
    #[characteristic(uuid = "2a4d", read, notify)]
    pub(crate) input_keyboard: [u8; 8],
    #[descriptor(uuid = "2908", read, value = [0u8, 2u8])]
    #[characteristic(uuid = "2a4d", read, write, write_without_response)]
    pub(crate) output_keyboard: u8,
}

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    info!("len: {}", KeypadReport::desc().len());

    info!("starting ble stack");
    let stack = ariel_os::ble::ble_stack().await;
    let mut peer = if let Some(bond) = ariel_os::ble::get_bonding_information().await {
        info!("Bond information: {:?} ", bond);
        let identity = bond.0.identity;
        stack.add_bond_information(bond.0).unwrap();
        Some(identity)
    } else {
        None
    };

    let mut host = stack.build();

    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: NAME,

        // Some Android devices don't like a keyboard with `trouble_host::IoCapabilities::DisplayOnly`
        // appearance: &appearance::human_interface_device::KEYBOARD,
        appearance: &appearance::human_interface_device::GENERIC_HUMAN_INTERFACE_DEVICE,
        // appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    }))
    .unwrap();

    info!("Using address: {}", ariel_os::ble::current_address().await);

    info!("Starting advertising");
    let _ = join(host.runner.run(), async {
        loop {
            let adv = advertise(NAME, &mut host.peripheral, &server, peer);

            let res = if peer.is_some() {
                let pairing = async {
                    loop {
                        let keycodes = KEYS_CHANNEL.receive().await;

                        if keycodes[0] != 0 {
                            match select(Timer::after_secs(2), KEYS_CHANNEL.receive()).await {
                                Either::First(_) => {
                                    if let Some(i) = peer.take() {
                                        let _ = ariel_os::ble::remove_bonding_information().await;
                                        let _ = stack.remove_bond_information(i);

                                        return;
                                    }
                                }
                                // havent presset for long enough
                                Either::Second(_) => {}
                            }
                        }
                    }
                };
                match select(adv, pairing).await {
                    Either::First(res) => res,
                    Either::Second(_) => continue,
                }
            } else {
                adv.await
            };

            match res {
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

                            let status = server.hid_service.output_keyboard.get(&server).unwrap();

                            info!("status : {}", status);

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

                    let res = embassy_futures::select::select(
                        gatt_events_task(&server, &conn, &mut peer),
                        keypad,
                    )
                    .await;

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
    peer: Option<Identity>,
) -> Result<GattConnection<'a, 'b, DefaultPacketPool>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 60];

    let len = if peer.is_some() {
        AdStructure::encode_slice(
            &[
                // AdStructure::CompleteLocalName(name.as_bytes()),
                AdStructure::Flags(BR_EDR_NOT_SUPPORTED),
                AdStructure::ServiceUuids16(&[
                    service::BATTERY.to_le_bytes(),
                    service::HUMAN_INTERFACE_DEVICE.to_le_bytes(),
                ]),
                // AdStructure::Unknown {
                //     ty: 0x19, // Appearance
                //     data: &appearance::human_interface_device::KEYBOARD.to_le_bytes(),
                // },
            ],
            &mut advertiser_data[..],
        )?
    } else {
        AdStructure::encode_slice(
            &[
                AdStructure::CompleteLocalName(name.as_bytes()),
                AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                AdStructure::ServiceUuids16(&[
                    service::BATTERY.to_le_bytes(),
                    service::HUMAN_INTERFACE_DEVICE.to_le_bytes(),
                ]),
                // AdStructure::Unknown {
                //     ty: 0x19, // Appearance
                //     data: &appearance::human_interface_device::KEYBOARD.to_le_bytes(),
                // },
            ],
            &mut advertiser_data[..],
        )?
    };

    let advertisement_data = Advertisement::ConnectableScannableUndirected {
        adv_data: &advertiser_data.get(..len).unwrap(),
        scan_data: &[],
    };

    let advertise_config = AdvertisementParameters {
        interval_min: Duration::from_millis(100),
        interval_max: Duration::from_millis(100),
        ..Default::default()
    };

    let advertiser = peripheral
        .advertise(&advertise_config, advertisement_data)
        .await?;
    info!("advertising");
    let conn = advertiser.accept().await?;

    conn.set_bondable(peer.is_none())?;

    // Usually the central sets up security but the peripheral can also request security.
    // if peer.is_none() {
    //     conn.request_security()?;
    // }

    let conn = conn.with_attribute_server(server)?;
    info!("connection established");
    Ok(conn)
}

/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_task(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, DefaultPacketPool>,

    peer: &mut Option<Identity>,
) -> Result<(), Error> {
    info!("hid_info handle : {}", server.hid_service.hid_info.handle);

    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            }

            GattConnectionEvent::PassKeyDisplay(key) => {
                info!("passkey: {}", key);
            }
            GattConnectionEvent::PassKeyInput => {
                info!("[gatt] passkey input");
                // Normally fetched from the user
            }
            GattConnectionEvent::PassKeyConfirm(key) => {
                info!("passkey confirm : {}", key);
            }
            GattConnectionEvent::PairingComplete {
                security_level,
                bond,
            } => {
                // TODO : handle bonding
                info!(
                    "Pairing complete, security level: {:?}, bond {:?}",
                    security_level, bond
                );

                if let Some(bond_information) = bond {
                    peer.replace(bond_information.identity);
                    ariel_os::ble::store_bonding_information(bond_information)
                        .await
                        .unwrap()
                }
            }
            GattConnectionEvent::PairingFailed(err) => {
                error!("Pairing failed: {:?}", err);
            }
            GattConnectionEvent::Gatt { event } => {
                if !conn.raw().security_level()?.encrypted() {
                    if let Ok(reply) = event.reject(AttErrorCode::INSUFFICIENT_ENCRYPTION) {
                        reply.send().await;
                    }
                    continue;
                }

                let payload = event.payload();

                let incoming = payload.incoming();
                info!("Gatt incoming: {:?}", incoming);

                match &event {
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

                if let Ok(reply) = event.accept() {
                    let _ = reply.send().await;
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

        if btn0.get_level() == Level::Low {
            keys[0] = 0x39;
        }
        KEYS_CHANNEL.send(keys).await;
    }
}

#[ariel_os::task(autostart, peripherals)]
async fn led(peripherals: pins::LedPeripherals) {
    let mut led0 = Output::new(peripherals.led0, Level::High);

    loop {
        let led_status = LEDS_CHANNEL.receive().await;

        info!("Received status: {:x}", led_status);

        if led_status & 0x02 == 0x02 {
            led0.set_low();
        } else {
            led0.set_high();
        }
    }
}
