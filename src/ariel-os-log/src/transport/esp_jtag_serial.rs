use core::cell::RefCell;

use embassy_sync::{
    blocking_mutex::{Mutex, raw::CriticalSectionRawMutex},
    once_lock::OnceLock,
};
use esp_hal::{Blocking, usb_serial_jtag::UsbSerialJtag};

static JTAG_SERIAL: OnceLock<Mutex<CriticalSectionRawMutex, RefCell<UsbSerialJtag<'_, Blocking>>>> =
    OnceLock::new();

pub(crate) fn write_bytes(bytes: &[u8]) {
    if let Some(mutex) = JTAG_SERIAL.try_get() {
        mutex.lock(|cell| {
            cell.borrow_mut().write(bytes);
        })
    }
}
pub(crate) fn flush() {
    if let Some(mutex) = JTAG_SERIAL.try_get() {
        let _ = mutex.lock(|cell| cell.borrow_mut().flush_tx());
    }
}

pub fn init(usb_device: esp_hal::peripherals::USB_DEVICE<'static>) {
    let _ = JTAG_SERIAL.init(Mutex::new(RefCell::new(UsbSerialJtag::new(usb_device))));
}
