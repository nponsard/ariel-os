#![no_std]
#![no_main]

use ariel_os::{
    log::{debug, error, info, println, trace, transport::register_custom_transport, warn},
    time::Timer,
};

#[cfg(context = "stm32")]
use embassy_stm32::{
    bind_interrupts,
    mode::Blocking,
    peripherals,
    usart::{BufferedUart, InterruptHandler},
};

#[cfg(context = "nrf")]
use embassy_nrf::{bind_interrupts, peripherals, uarte};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, pipe::Pipe};
use embedded_io::Write;
use static_cell::ConstStaticCell;

const MAX_FULL_SPEED_PACKET_SIZE: u8 = 64;

const LOG_BUFFER_SIZE: usize = 1024;
static LOG_BUFFER: Pipe<CriticalSectionRawMutex, LOG_BUFFER_SIZE> = Pipe::new();

#[ariel_os::task(autostart)]
async fn logging() {
    loop {
        Timer::after_secs(1).await;
        info!("Testing USB CDC ACM logging");
        println!("-- this is printed via `println!()`");
        trace!("-- trace log level enabled");
        debug!("-- debug log level enabled");
        info!("-- info log level enabled");
        warn!("-- warn log level enabled");
        error!("-- error log level enabled (just testing)");
    }
}

#[cfg(context = "stm32")]
type Uart = embassy_stm32::usart::Uart<'static, Blocking>;

#[cfg(context = "nrf")]
type Uart = embassy_nrf::uarte::Uarte<'static>;

static UART: Mutex<CriticalSectionRawMutex, Option<Uart>> = Mutex::new(None);

// Side UART of Arduino v3 connector
#[cfg(context = "stm32u083c-dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA3,
    uart_tx: PA2,
    uart: USART2,
});

#[cfg(context = "stm32u083c-dk")]
bind_interrupts!(struct Irqs {
    USART2_LPUART2 => InterruptHandler<peripherals::USART2>;
});

#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_08,
    uart_tx: P0_06,
    uart: UARTE0,
});

#[cfg(context = "nrf52840dk")]
bind_interrupts!(struct Irqs {
    UARTE0 => uarte::InterruptHandler<peripherals::UARTE0>;
});

const UART_BUF_SIZE: usize = 32;
static RX_BUF: ConstStaticCell<[u8; UART_BUF_SIZE]> = ConstStaticCell::new([0u8; UART_BUF_SIZE]);
static TX_BUF: ConstStaticCell<[u8; UART_BUF_SIZE]> = ConstStaticCell::new([0u8; UART_BUF_SIZE]);

#[ariel_os::task(autostart, peripherals)]
async fn main(p: Peripherals) {
    register_custom_transport(write_bytes, flush);
    let tx_buf = TX_BUF.take();
    let rx_buf = RX_BUF.take();

    #[cfg(context = "stm32")]
    let config = embassy_stm32::usart::Config::default();
    #[cfg(context = "nrf")]
    let config = embassy_nrf::uarte::Config::default();
    if let Ok(mut l) = UART.try_lock() {
        #[cfg(context = "stm32")]
        let uart = Uart::new_blocking(p.uart, p.uart_rx, p.uart_tx, config).unwrap();
        #[cfg(context = "nrf")]
        let uart = Uart::new(p.uart, p.uart_rx, p.uart_tx, Irqs, config);

        l.replace(uart);
    }

    info!("Hello world!");
}

pub fn write_bytes(bytes: &[u8]) {
    let end = bytes.len();

    if let Ok(mut opt) = UART.try_lock() {
        if let Some(mut uart) = opt.take() {
            let res = cfg_select! {
                context = "nrf"=> {
                    uart.blocking_write(bytes)
                }
                _ => {
                    uart.write_all(bytes)
                }
            };

            if let Err(e) = res {
                panic!("aaaa");
            }

            opt.replace(uart);
        } else {
            panic!("aaaa");
        }
    } else {
        panic!("aaaa");
    }
}

// No-op, flushing isn't possible with this setup.
pub fn flush() {}
