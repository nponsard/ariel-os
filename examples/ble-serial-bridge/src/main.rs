//! Adapted from the example in `trouble_host`
#![no_main]
#![no_std]

mod pins;

use embassy_futures::{
    join::join,
    select::{Either, select},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embedded_io_async::{Read, Write as _};
use heapless::Vec;
use trouble_host::{
    BleHostError, Controller, Error,
    advertise::{AdStructure, Advertisement, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE},
    gap::{GapConfig, PeripheralConfig},
    gatt::{GattConnection, GattConnectionEvent, GattEvent},
    prelude::{FromGatt, Peripheral, appearance, gatt_server, gatt_service},
};

use ariel_os::{
    debug::log::{info, warn},
    hal,
    uart::Baudrate,
};

use crate::pins::Peripherals;
const MAX_TX_PACKET_SIZE: usize = 20;
const MAX_RX_PACKET_SIZE: usize = 20;

static RX_CHANNEL: Channel<CriticalSectionRawMutex, Vec<u8, MAX_RX_PACKET_SIZE>, 8> =
    Channel::new();
static TX_CHANNEL: Channel<CriticalSectionRawMutex, Vec<u8, MAX_TX_PACKET_SIZE>, 8> =
    Channel::new();

// GATT Server definition
#[gatt_server]
struct Server {
    uart_service: UartService,
}

#[gatt_service(uuid = "8ea9309a-c13a-4038-8460-7dccea6d7b20")]
struct UartService {
    /// Fake Nordic UART write characteristic
    #[characteristic(uuid = "6e400002-b5a3-f393-e0a9-e50e24dcca9e", write_without_response)]
    write_data: [u8; MAX_TX_PACKET_SIZE],
    /// Fake Nordic UART notify characteristic
    #[characteristic(uuid = "6e400003-b5a3-f393-e0a9-e50e24dcca9e", read, notify)]
    read_data: u8,
}

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    info!("starting ble stack");

    let stack = ariel_os::ble::ble_stack().await;
    let mut host = stack.build();

    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        name: "Ariel BLE UART bridge",
        appearance: &appearance::motorized_vehicle::TROLLEY,
    }))
    .unwrap();

    info!("Starting advertising");
    let _ = join(host.runner.run(), async {
        loop {
            match advertise("Ariel OS", &mut host.peripheral, &server).await {
                Ok(conn) => {
                    // set up tasks when the connection is established to a central, so they don't run when no one is connected.
                    let a = gatt_events_task(&server, &conn);
                    let b = ble_tx(&server, &conn);
                    // run until any task ends (usually because the connection has been closed),
                    // then return to advertising state.
                    select(a, b).await;
                }
                Err(e) => {
                    panic!("[adv] error: {:?}", e);
                }
            }
        }
    })
    .await;
}

#[ariel_os::task(autostart, peripherals)]
async fn uart_runner(peripherals: Peripherals) {
    let mut config = hal::uart::Config::default();
    config.baudrate = Baudrate::_115200;
    info!("Selected configuration: {:?}", config);

    let mut rx_buf = [0u8; 32];
    let mut tx_buf = [0u8; 32];

    let mut uart = pins::TestUart::new(
        peripherals.uart_rx,
        peripherals.uart_tx,
        &mut rx_buf,
        &mut tx_buf,
        config,
    )
    .expect("Invalid UART configuration");

    loop {
        let mut buf = [0u8; MAX_RX_PACKET_SIZE];

        let read = async { uart.read(&mut buf).await.expect("Failed to read") };
        let write = async {
            TX_CHANNEL.ready_to_receive().await;
        };

        match select(write, read).await {
            Either::First(_) => {
                let data = TX_CHANNEL.receive().await;
                uart.write(&data).await.expect("UART write error");
            }
            Either::Second(n) => {
                let mut data: Vec<u8, MAX_RX_PACKET_SIZE> = Vec::new();
                data.extend_from_slice(&buf[..n]).unwrap();
                RX_CHANNEL.send(data).await;
            }
        }
    }
}

/// Stream Events until the connection closes.
///
/// This function will handle the GATT events and process them.
/// This is how we interact with read and write requests.
async fn gatt_events_task(server: &Server<'_>, conn: &GattConnection<'_, '_>) -> Result<(), Error> {
    let write_data = server.uart_service.write_data;
    loop {
        match conn.next().await {
            GattConnectionEvent::Bonded { bond_info } => {
                info!("[gatt] pairing complete: {:?}", bond_info);

            }
            GattConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => match event {
                Ok(event) => {
                    match &event {
                        GattEvent::Read(event) => {
                            warn!("[gatt] Read Event to Characteristic: {:?}", event.handle());
                        }
                        GattEvent::Write(event) => {
                            if event.handle() == write_data.handle {
                                let data = event.data();
                                info!("[gatt] Write Event to Characteristic: {:?}", data);
                                let len = data.len().min(MAX_TX_PACKET_SIZE);
                                let mut vec = Vec::<u8, MAX_TX_PACKET_SIZE>::new();

                                let err = vec.extend_from_slice(&data[..len]);
                                if err.is_err() {
                                    warn!("[gatt] error extending vec, dropping packet");
                                } else {
                                    TX_CHANNEL.send(vec).await;
                                }
                            }
                        }
                    }

                    // This step is also performed at drop(), but writing it explicitly is necessary
                    // in order to ensure reply is sent.
                    match event.accept() {
                        Ok(reply) => {
                            reply.send().await;
                        }
                        Err(e) => warn!("[gatt] error sending response: {:?}", e),
                    }
                }
                Err(e) => warn!("[gatt] error processing event: {:?}", e),
            },
            _ => {}
        }
    }
    info!("[gatt] task finished");
    Ok(())
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, 'b, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
    server: &'b Server<'_>,
) -> Result<GattConnection<'a, 'b>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            // AdStructure::ServiceUuids16(&[[0x0f, 0x18]]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    info!("[adv] connection established");
    Ok(conn)
}

/// Send the data received from UART to the connected BLE Central.
async fn ble_tx(server: &Server<'_>, conn: &GattConnection<'_, '_>) {
    let read_data = server.uart_service.read_data;
    loop {
        let data = RX_CHANNEL.receive().await;

        info!(
            "[ble_tx] notifying connection of new data: {:?}",
            defmt::Debug2Format(&data)
        );
        for b in data {
            if let Err(e) = read_data.notify(conn, &b).await {
                info!("[ble_tx] error notifying connection : {:?}", e);
                break;
            };
        }
    }
}
