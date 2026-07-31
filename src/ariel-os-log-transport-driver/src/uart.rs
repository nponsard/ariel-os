use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embedded_io_async_06::Write;
use static_cell::ConstStaticCell;

use ariel_os_embassy_common::uart::{Assignment, Baudrate};
use ariel_os_hal::hal::{OptionalPeripherals, TakePeripherals, uart::Uart};

type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

static WRITER: Mutex<CriticalSectionRawMutex, Option<Uart<'static>>> = Mutex::new(None);

static RX_BUF: ConstStaticCell<[u8; 32]> = ConstStaticCell::new([0u8; 32]);
static TX_BUF: ConstStaticCell<[u8; 32]> = ConstStaticCell::new([0u8; 32]);

/// Initialize UART log transport.
pub fn init(mut peripherals: &mut OptionalPeripherals, _spawner: Spawner) {
    let mut config = ariel_os_hal::hal::uart::Config::default();
    config.baudrate = Baudrate::_115200;

    let tx_buf = TX_BUF.take();
    let rx_buf = RX_BUF.take();

    let assignment: UartAssignment = peripherals.take_peripherals();

    let (tx, rx) = assignment.into_pins();

    let uart = <UartAssignment as Assignment>::Device::new(rx, tx, rx_buf, tx_buf, config).unwrap();

    let mut writer = WRITER.try_lock().unwrap();
    writer.replace(uart);

    ariel_os_log::custom_transport::register_transport_functions(write_bytes, flush);
}

pub(crate) fn write_bytes(bytes: &[u8]) {
    if let Ok(mut opt) = WRITER.try_lock()
        && let Some(writer) = opt.as_mut()
    {
        let _ = embassy_futures::block_on(writer.write_all(bytes));
    }
}

pub(crate) fn flush() {
    if let Ok(mut opt) = WRITER.try_lock()
        && let Some(writer) = opt.as_mut()
    {
        let _ = embassy_futures::block_on(writer.flush());
    }
}
