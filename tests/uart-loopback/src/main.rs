//! This is a test for UART loopback operation.

#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::{ExitCode, exit},
    hal,
    log::{Hex, info},
    time::{Duration, with_timeout},
    uart::Baudrate,
};

use embedded_io_async::{Read as _, Write as _};

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    info!("Starting UART test");

    let mut config = hal::uart::Config::default();
    let mut config2 = hal::uart::Config::default();

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
    let mut rx_buf2 = [0u8; 32];
    let mut tx_buf2 = [0u8; 32];

    let mut uart2 = pins::TestUart::new(
        peripherals.uart_rx2,
        peripherals.uart_tx2,
        &mut rx_buf2,
        &mut tx_buf2,
        config2,
    )
    .expect("Invalid UART configuration");

    const OUT: &str = "Test Message";
    const OUT2: &str = "Test 22222";

    let mut input = [0u8; OUT.len()];
    let mut input2 = [0u8; OUT2.len()];

    uart.write_all(OUT.as_bytes()).await.unwrap();
    uart2.write_all(OUT2.as_bytes()).await.unwrap();
    uart.flush().await.unwrap();
    uart2.flush().await.unwrap();
    info!("Wrote bytes");
    let _ = with_timeout(Duration::from_secs(5), uart2.read_exact(&mut input2)).await;
    let _ = with_timeout(Duration::from_secs(5), uart.read_exact(&mut input)).await;

    info!("Got: {}", Hex(input));
    assert_eq!(OUT.as_bytes(), input);
    info!("Got: {}", Hex(input2));
    assert_eq!(OUT2.as_bytes(), input2);
    info!("Test passed!");

    exit(ExitCode::SUCCESS);
}
