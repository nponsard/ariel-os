// modification of https://github.com/embassy-rs/embassy/blob/main/examples/nrf9160/src/bin/modem_tcp_client.rs

#![no_main]
#![no_std]

use core::str::FromStr as _;

use embedded_io_async::Write as _;

use ariel_os::time::Timer;
use ariel_os::{
    log::{info, warn},
    net,
    reexports::embassy_net,
    time::Duration,
};

#[cfg(context = "nordic-thingy-91-x-nrf9151")]
use ariel_os::hal::peripherals;
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
pub type SensorI2c = ariel_os::hal::i2c::controller::SERIAL1;
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: P0_09,
    i2c_scl: P0_08,
});

#[cfg(not(context = "nordic-thingy-91-x-nrf9151"))]
ariel_os::hal::define_peripherals!(Peripherals {});

#[cfg(context = "nordic-thingy-91-x-nrf9151")]
#[ariel_os::task(autostart, peripherals)]
async fn board_init(peripherals: Peripherals) {
    use ariel_os::{
        i2c::controller::{Kilohertz, highest_freq_in},
        log::Debug2Format,
    };
    let mut i2c_config = ariel_os::hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };

    let mut i2c_bus = SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);

    let err = ariel_os::hal::boards::init_thingy91x_board(&mut i2c_bus, true, true).await;
    info!("{:?}", Debug2Format(&err));
}

#[ariel_os::task(autostart)]
async fn tcp_echo() {
    let stack = net::network_stack().await.unwrap();

    // Increase the buffer size if you want to send bigger packets.
    let mut rx_buffer = [0; 256];
    let mut tx_buffer = [0; 256];

    info!("waiting for interface to come up...");
    stack.wait_config_up().await;

    loop {
        let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        info!("Connecting...");
        // Connect to https://tcpbin.com/ without using DNS
        let host_addr = embassy_net::Ipv4Address::from_str("45.79.112.203").unwrap();
        if let Err(e) = socket.connect((host_addr, 4242)).await {
            warn!("connect error: {:?}", e);
            Timer::after_secs(10).await;
            continue;
        }
        info!("Connected to {:?}", socket.remote_endpoint());

        let msg = b"Hello world!\n";
        for _ in 0..10 {
            #[allow(unused_variables, reason = "log macro sometimes doesn't use this")]
            if let Err(e) = socket.write_all(msg).await {
                warn!("write error: {:?}", e);
                break;
            }
            // We put trust in the server to return a valid utf8 string
            info!("txd: {}", core::str::from_utf8(msg).unwrap());
            Timer::after_secs(1).await;
        }

        Timer::after_secs(4).await;
    }
}
