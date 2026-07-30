use core::cell::RefCell;

use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};
use esp_hal::{Blocking, usb_serial_jtag::UsbSerialJtag};

// Limit the amount of locks, we don't use a OnceLock here.
static JTAG_SERIAL: Mutex<CriticalSectionRawMutex, RefCell<Option<UsbSerialJtag<'_, Blocking>>>> =
    Mutex::new(RefCell::new(None));

pub(crate) fn write_bytes(bytes: &[u8]) {
    JTAG_SERIAL.lock(|cell| {
        if let Some(serial) = cell.borrow_mut().as_mut() {
            serial.write(bytes);
        }
    })
}
pub(crate) fn flush() {
    JTAG_SERIAL.lock(|cell| {
        if let Some(serial) = cell.borrow_mut().as_mut() {
            serial.flush_tx();
        }
    });
}

pub fn init(usb_device: esp_hal::peripherals::USB_DEVICE<'static>) {
    let _ = JTAG_SERIAL.lock(|cell| cell.replace(Some(UsbSerialJtag::new(usb_device))));
}
